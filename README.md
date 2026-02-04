# RustyCurrency

A REST API built with Rust and Axum framework that provides country information including flags and currency codes.

## Features

- Get country information by country name
- Support for multiple countries in a single request (comma-separated)
- Returns country flag emoji and currency code
- Case-insensitive country name lookup
- Comprehensive test suite

## Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

Clone the repository and build:

```bash
cargo build --release
```

## Running the Server

```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`

## Testing

The project includes a comprehensive test suite that covers all API functionality.

### Run all tests

```bash
cargo test
```

### Run tests with output

```bash
cargo test -- --nocapture
```

### Run a specific test

```bash
cargo test test_get_country_single
```

### Test Coverage

The test suite includes:
- ✅ Single country queries
- ✅ Multiple country queries (comma-separated)
- ✅ Case-insensitive lookups
- ✅ Unknown country handling
- ✅ Whitespace handling
- ✅ Mixed valid/invalid country queries
- ✅ Multiple supported countries

## API Usage

### Endpoint: `/getCountry`

**Method:** GET

**Query Parameter:** `based` - Country name(s), comma-separated for multiple countries

### Examples

#### Single Country
```bash
curl "http://localhost:3000/getCountry?based=japan"
```

Response:
```json
{
  "results": [
    {
      "country": "japan",
      "flag": "🇯🇵",
      "currencyCode": "JPY"
    }
  ]
}
```

#### Multiple Countries
```bash
curl "http://localhost:3000/getCountry?based=japan,korea"
```

Response:
```json
{
  "results": [
    {
      "country": "japan",
      "flag": "🇯🇵",
      "currencyCode": "JPY"
    },
    {
      "country": "korea",
      "flag": "🇰🇷",
      "currencyCode": "KRW"
    }
  ]
}
```

## Supported Countries

- Japan (🇯🇵 - JPY)
- Korea/South Korea (🇰🇷 - KRW)
- United States/USA (🇺🇸 - USD)
- United Kingdom/UK (🇬🇧 - GBP)
- China (🇨🇳 - CNY)
- Germany (🇩🇪 - EUR)
- France (🇫🇷 - EUR)
- Canada (🇨🇦 - CAD)
- Australia (🇦🇺 - AUD)
- Brazil (🇧🇷 - BRL)
- India (🇮🇳 - INR)
- Mexico (🇲🇽 - MXN)
- Singapore (🇸🇬 - SGD)
- Switzerland (🇨🇭 - CHF)
- Sweden (🇸🇪 - SEK)
- Norway (🇳🇴 - NOK)
- Denmark (🇩🇰 - DKK)

## Technologies Used

- **Rust** - Programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization