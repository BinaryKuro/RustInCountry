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
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_country_single() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 1);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[0].flag, "🇯🇵");
        assert_eq!(country_response.results[0].currency_code, "JPY");
        assert_eq!(country_response.results[0].phone_code, "+81");
    }

    #[tokio::test]
    async fn test_get_country_multiple() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan,korea")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 2);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[0].flag, "🇯🇵");
        assert_eq!(country_response.results[0].currency_code, "JPY");
        assert_eq!(country_response.results[0].phone_code, "+81");
        assert_eq!(country_response.results[1].country, "korea");
        assert_eq!(country_response.results[1].flag, "🇰🇷");
        assert_eq!(country_response.results[1].currency_code, "KRW");
        assert_eq!(country_response.results[1].phone_code, "+82");
    }

    #[tokio::test]
    async fn test_get_country_case_insensitive() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=JAPAN")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 1);
        assert_eq!(country_response.results[0].country, "JAPAN");
        assert_eq!(country_response.results[0].flag, "🇯🇵");
        assert_eq!(country_response.results[0].currency_code, "JPY");
        assert_eq!(country_response.results[0].phone_code, "+81");
    }

    #[tokio::test]
    async fn test_get_country_unknown() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=unknown")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 0);
    }

    #[tokio::test]
    async fn test_get_country_with_spaces() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan,%20korea,%20usa")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 3);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[1].country, "korea");
        assert_eq!(country_response.results[2].country, "usa");
    }

    #[tokio::test]
    async fn test_get_country_mixed_valid_invalid() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan,unknown,korea")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        // Should only return valid countries
        assert_eq!(country_response.results.len(), 2);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[1].country, "korea");
    }

    #[tokio::test]
    async fn test_get_country_all_supported() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=usa,uk,germany")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 3);
        assert_eq!(country_response.results[0].flag, "🇺🇸");
        assert_eq!(country_response.results[0].currency_code, "USD");
        assert_eq!(country_response.results[0].phone_code, "+1");
        assert_eq!(country_response.results[1].flag, "🇬🇧");
        assert_eq!(country_response.results[1].currency_code, "GBP");
        assert_eq!(country_response.results[1].phone_code, "+44");
        assert_eq!(country_response.results[2].flag, "🇩🇪");
        assert_eq!(country_response.results[2].currency_code, "EUR");
        assert_eq!(country_response.results[2].phone_code, "+49");
    }

    #[tokio::test]
    async fn test_get_country_comprehensive_coverage() {
        let app = create_app();

        // Test countries from various continents
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=afghanistan,argentina,egypt,fiji,iceland,nigeria,thailand")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 7);
        
        // Afghanistan (Asia)
        assert_eq!(country_response.results[0].country, "afghanistan");
        assert_eq!(country_response.results[0].flag, "🇦🇫");
        assert_eq!(country_response.results[0].currency_code, "AFN");
        assert_eq!(country_response.results[0].phone_code, "+93");
        
        // Argentina (South America)
        assert_eq!(country_response.results[1].country, "argentina");
        assert_eq!(country_response.results[1].flag, "🇦🇷");
        assert_eq!(country_response.results[1].currency_code, "ARS");
        
        // Egypt (Africa)
        assert_eq!(country_response.results[2].country, "egypt");
        assert_eq!(country_response.results[2].flag, "🇪🇬");
        assert_eq!(country_response.results[2].currency_code, "EGP");
        assert_eq!(country_response.results[2].phone_code, "+20");
        
        // Fiji (Oceania)
        assert_eq!(country_response.results[3].country, "fiji");
        assert_eq!(country_response.results[3].flag, "🇫🇯");
        assert_eq!(country_response.results[3].currency_code, "FJD");
        assert_eq!(country_response.results[3].phone_code, "+679");
        
        // Iceland (Europe)
        assert_eq!(country_response.results[4].country, "iceland");
        assert_eq!(country_response.results[4].flag, "🇮🇸");
        assert_eq!(country_response.results[4].currency_code, "ISK");
        assert_eq!(country_response.results[4].phone_code, "+354");
        
        // Nigeria (Africa)
        assert_eq!(country_response.results[5].country, "nigeria");
        assert_eq!(country_response.results[5].flag, "🇳🇬");
        assert_eq!(country_response.results[5].currency_code, "NGN");
        assert_eq!(country_response.results[5].phone_code, "+234");
        
        // Thailand (Asia)
        assert_eq!(country_response.results[6].country, "thailand");
        assert_eq!(country_response.results[6].flag, "🇹🇭");
        assert_eq!(country_response.results[6].currency_code, "THB");
        assert_eq!(country_response.results[6].phone_code, "+66");
    }

    #[tokio::test]
    async fn test_get_country_african_countries() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=kenya,morocco,ghana,ethiopia")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 4);
        assert_eq!(country_response.results[0].flag, "🇰🇪");
        assert_eq!(country_response.results[0].currency_code, "KES");
        assert_eq!(country_response.results[0].phone_code, "+254");
        assert_eq!(country_response.results[1].flag, "🇲🇦");
        assert_eq!(country_response.results[1].currency_code, "MAD");
        assert_eq!(country_response.results[1].phone_code, "+212");
    }

    #[tokio::test]
    async fn test_get_country_aliases() {
        let app = create_app();

        // Test common aliases
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=uae,czechia,burma,vatican")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 4);
        assert_eq!(country_response.results[0].flag, "🇦🇪"); // UAE
        assert_eq!(country_response.results[0].currency_code, "AED");
        assert_eq!(country_response.results[0].phone_code, "+971");
        assert_eq!(country_response.results[1].flag, "🇨🇿"); // Czechia
        assert_eq!(country_response.results[1].currency_code, "CZK");
        assert_eq!(country_response.results[1].phone_code, "+420");
        assert_eq!(country_response.results[2].flag, "🇲🇲"); // Burma/Myanmar
        assert_eq!(country_response.results[2].currency_code, "MMK");
        assert_eq!(country_response.results[2].phone_code, "+95");
        assert_eq!(country_response.results[3].flag, "🇻🇦"); // Vatican
        assert_eq!(country_response.results[3].currency_code, "EUR");
        assert_eq!(country_response.results[3].phone_code, "+3906698");
    }

    #[test]
    fn test_parse_country_data_skips_malformed_lines() {
        let csv_data =
            "country,flag,currencyCode,phoneCode\nvalid,🏳️,VAL,+999\nmissing-flag,,MFG,+000\nmissing-code,🏳️,,+000\nmissing-phone,🏳️,VAL,\n";
        let reader = std::io::BufReader::new(csv_data.as_bytes());
        let data = parse_country_data(reader);

        assert_eq!(data.len(), 1);
        assert_eq!(
            data.get("valid"),
            Some(&(
                String::from("🏳️"),
                String::from("VAL"),
                String::from("+999")
            ))
        );
    }

    #[tokio::test]
    async fn test_get_country_all_parameter() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=all")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), COUNTRY_DATA.len());
        assert!(country_response
            .results
            .iter()
            .any(|country| country.country == "japan" && country.phone_code == "+81"));
    }
}
