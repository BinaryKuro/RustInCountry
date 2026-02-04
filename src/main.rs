use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CountryQuery {
    based: String,
}

#[derive(Debug, Serialize)]
struct CountryInfo {
    country: String,
    flag: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[derive(Debug, Serialize)]
struct CountryResponse {
    results: Vec<CountryInfo>,
}

// Global country data initialized once
static COUNTRY_DATA: Lazy<HashMap<String, (String, String)>> = Lazy::new(|| {
    let mut data = HashMap::new();
    
    // Format: (flag emoji, currency code)
    data.insert("japan".to_string(), ("🇯🇵".to_string(), "JPY".to_string()));
    data.insert("korea".to_string(), ("🇰🇷".to_string(), "KRW".to_string()));
    data.insert("south korea".to_string(), ("🇰🇷".to_string(), "KRW".to_string()));
    data.insert("united states".to_string(), ("🇺🇸".to_string(), "USD".to_string()));
    data.insert("usa".to_string(), ("🇺🇸".to_string(), "USD".to_string()));
    data.insert("united kingdom".to_string(), ("🇬🇧".to_string(), "GBP".to_string()));
    data.insert("uk".to_string(), ("🇬🇧".to_string(), "GBP".to_string()));
    data.insert("china".to_string(), ("🇨🇳".to_string(), "CNY".to_string()));
    data.insert("germany".to_string(), ("🇩🇪".to_string(), "EUR".to_string()));
    data.insert("france".to_string(), ("🇫🇷".to_string(), "EUR".to_string()));
    data.insert("canada".to_string(), ("🇨🇦".to_string(), "CAD".to_string()));
    data.insert("australia".to_string(), ("🇦🇺".to_string(), "AUD".to_string()));
    data.insert("brazil".to_string(), ("🇧🇷".to_string(), "BRL".to_string()));
    data.insert("india".to_string(), ("🇮🇳".to_string(), "INR".to_string()));
    data.insert("mexico".to_string(), ("🇲🇽".to_string(), "MXN".to_string()));
    data.insert("singapore".to_string(), ("🇸🇬".to_string(), "SGD".to_string()));
    data.insert("switzerland".to_string(), ("🇨🇭".to_string(), "CHF".to_string()));
    data.insert("sweden".to_string(), ("🇸🇪".to_string(), "SEK".to_string()));
    data.insert("norway".to_string(), ("🇳🇴".to_string(), "NOK".to_string()));
    data.insert("denmark".to_string(), ("🇩🇰".to_string(), "DKK".to_string()));
    
    data
});

async fn get_country(Query(params): Query<CountryQuery>) -> Json<CountryResponse> {
    let mut results = Vec::new();
    
    // Split the based parameter by comma and process each country
    let countries: Vec<&str> = params.based.split(',').map(|s| s.trim()).collect();
    
    for country_name in countries {
        let country_lower = country_name.to_lowercase();
        
        if let Some((flag, currency_code)) = COUNTRY_DATA.get(&country_lower) {
            results.push(CountryInfo {
                country: country_name.to_string(),
                flag: flag.clone(),
                currency_code: currency_code.clone(),
            });
        }
    }
    
    Json(CountryResponse { results })
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new().route("/getCountry", get(get_country));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to 0.0.0.0:3000");
    
    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
