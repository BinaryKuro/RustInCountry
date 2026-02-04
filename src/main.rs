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

#[derive(Debug, Serialize, Deserialize)]
struct CountryInfo {
    country: String,
    flag: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
        assert_eq!(country_response.results[1].country, "korea");
        assert_eq!(country_response.results[1].flag, "🇰🇷");
        assert_eq!(country_response.results[1].currency_code, "KRW");
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
        assert_eq!(country_response.results[1].flag, "🇬🇧");
        assert_eq!(country_response.results[1].currency_code, "GBP");
        assert_eq!(country_response.results[2].flag, "🇩🇪");
        assert_eq!(country_response.results[2].currency_code, "EUR");
    }
}
