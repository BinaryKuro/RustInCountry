use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Deserialize)]
struct CountryQuery {
    based: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountryInfo {
    country: String,
    flag: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
    #[serde(rename = "phoneCode")]
    phone_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountryResponse {
    results: Vec<CountryInfo>,
}

const DEFAULT_COUNTRY_DATA_PATH: &str = "data/countries.csv";
const CSV_FIELD_COUNT: usize = 4;

fn parse_country_data<R: BufRead>(reader: R) -> HashMap<String, (String, String, String)> {
    let mut data = HashMap::new();

    for (line_index, line) in reader.lines().skip(1).enumerate() {
        let file_line_number = line_index + 2;
        let line = line.unwrap_or_else(|error| {
            panic!(
                "Failed to read country data line {}: {}",
                file_line_number, error
            );
        });
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(CSV_FIELD_COUNT, ',');
        let country = parts.next().unwrap_or("").trim();
        let flag = parts.next().unwrap_or("").trim();
        let currency_code = parts.next().unwrap_or("").trim();
        let phone_code = parts.next().unwrap_or("").trim();
        if country.is_empty() || flag.is_empty() || currency_code.is_empty() || phone_code.is_empty() {
            tracing::warn!(
                "Skipping malformed country data line {}: {}",
                file_line_number,
                line
            );
            continue;
        }
        data.insert(
            country.to_string(),
            (
                flag.to_string(),
                currency_code.to_string(),
                phone_code.to_string(),
            ),
        );
    }

    data
}

// Global country data initialized once - All 195 UN-recognized countries
static COUNTRY_DATA: Lazy<HashMap<String, (String, String, String)>> = Lazy::new(|| {
    let (path, path_source) = match std::env::var("COUNTRY_DATA_PATH") {
        Ok(path) => (path, "COUNTRY_DATA_PATH"),
        Err(_) => (DEFAULT_COUNTRY_DATA_PATH.to_string(), "default path"),
    };
    let file = File::open(&path).unwrap_or_else(|error| {
        panic!(
            "Failed to open country data file at {} (source: {}): {}",
            path, path_source, error
        )
    });
    let reader = BufReader::new(file);
    parse_country_data(reader)
});

async fn get_country(Query(params): Query<CountryQuery>) -> Json<CountryResponse> {
    let mut results = Vec::new();

    if params.based.trim().eq_ignore_ascii_case("all") {
        let mut countries: Vec<_> = COUNTRY_DATA.iter().collect();
        countries.sort_by(|(left, _), (right, _)| left.cmp(right));
        for (country_name, (flag, currency_code, phone_code)) in countries {
            results.push(CountryInfo {
                country: country_name.clone(),
                flag: flag.clone(),
                currency_code: currency_code.clone(),
                phone_code: phone_code.clone(),
            });
        }
        return Json(CountryResponse { results });
    }

    // Split the based parameter by comma and process each country
    let countries: Vec<&str> = params.based.split(',').map(|s| s.trim()).collect();

    for country_name in countries {
        let country_lower = country_name.to_lowercase();

        if let Some((flag, currency_code, phone_code)) = COUNTRY_DATA.get(&country_lower) {
            results.push(CountryInfo {
                country: country_name.to_string(),
                flag: flag.clone(),
                currency_code: currency_code.clone(),
                phone_code: phone_code.clone(),
            });
        }
    }

    Json(CountryResponse { results })
}

// Separate function to create the app router for testing
fn create_app() -> Router {
    Router::new().route("/getCountry", get(get_country))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = create_app();

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to 0.0.0.0:3000");
    
    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[cfg(test)]
mod tests;
