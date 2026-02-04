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

    assert_eq!(country_response.results[0].country, "afghanistan");
    assert_eq!(country_response.results[0].flag, "🇦🇫");
    assert_eq!(country_response.results[0].currency_code, "AFN");
    assert_eq!(country_response.results[0].phone_code, "+93");

    assert_eq!(country_response.results[1].country, "argentina");
    assert_eq!(country_response.results[1].flag, "🇦🇷");
    assert_eq!(country_response.results[1].currency_code, "ARS");

    assert_eq!(country_response.results[2].country, "egypt");
    assert_eq!(country_response.results[2].flag, "🇪🇬");
    assert_eq!(country_response.results[2].currency_code, "EGP");
    assert_eq!(country_response.results[2].phone_code, "+20");

    assert_eq!(country_response.results[3].country, "fiji");
    assert_eq!(country_response.results[3].flag, "🇫🇯");
    assert_eq!(country_response.results[3].currency_code, "FJD");
    assert_eq!(country_response.results[3].phone_code, "+679");

    assert_eq!(country_response.results[4].country, "iceland");
    assert_eq!(country_response.results[4].flag, "🇮🇸");
    assert_eq!(country_response.results[4].currency_code, "ISK");
    assert_eq!(country_response.results[4].phone_code, "+354");

    assert_eq!(country_response.results[5].country, "nigeria");
    assert_eq!(country_response.results[5].flag, "🇳🇬");
    assert_eq!(country_response.results[5].currency_code, "NGN");
    assert_eq!(country_response.results[5].phone_code, "+234");

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
