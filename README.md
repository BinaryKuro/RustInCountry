# RustyCurrency

A REST API built with Rust and Axum framework that provides country information including flags and currency codes for **all 195 UN-recognized countries**.

## Features

- Get country information by country name
- Support for multiple countries in a single request (comma-separated)
- Returns country flag emoji, currency code, and phone code
- Case-insensitive country name lookup
- **Complete coverage of all 195 UN-recognized countries**
- Support for common country name aliases (e.g., USA/United States, UAE, Czechia, etc.)
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
- ✅ Comprehensive coverage tests (countries from all continents)
- ✅ African countries coverage
- ✅ Country name aliases

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
       "currencyCode": "JPY",
       "phoneCode": "+81"
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
       "currencyCode": "JPY",
       "phoneCode": "+81"
     },
     {
       "country": "korea",
       "flag": "🇰🇷",
       "currencyCode": "KRW",
       "phoneCode": "+82"
     }
  ]
}
```

#### All Countries
```bash
curl "http://localhost:3000/getCountry?based=all"
```

Response (truncated):
```json
{
  "results": [
    {
      "country": "afghanistan",
      "flag": "🇦🇫",
      "currencyCode": "AFN",
      "phoneCode": "+93"
    }
  ]
}
```

## Supported Countries

**All 195 UN-recognized countries are supported!** 🌍

The API includes support for all countries from every continent:

### Examples by Region

**Asia**: Afghanistan, Bangladesh, China, India, Indonesia, Iran, Iraq, Israel, Japan, Jordan, Kazakhstan, Kuwait, Laos, Lebanon, Malaysia, Mongolia, Myanmar (Burma), Nepal, North Korea, Oman, Pakistan, Palestine, Philippines, Qatar, Saudi Arabia, Singapore, South Korea, Sri Lanka, Syria, Tajikistan, Thailand, Turkey, Turkmenistan, UAE, Uzbekistan, Vietnam, Yemen

**Europe**: Albania, Andorra, Armenia, Austria, Azerbaijan, Belarus, Belgium, Bosnia and Herzegovina, Bulgaria, Croatia, Cyprus, Czech Republic (Czechia), Denmark, Estonia, Finland, France, Georgia, Germany, Greece, Hungary, Iceland, Ireland, Italy, Latvia, Liechtenstein, Lithuania, Luxembourg, Malta, Moldova, Monaco, Montenegro, Netherlands, North Macedonia, Norway, Poland, Portugal, Romania, Russia, San Marino, Serbia, Slovakia, Slovenia, Spain, Sweden, Switzerland, Ukraine, United Kingdom (UK), Vatican City

**Africa**: Algeria, Angola, Benin, Botswana, Burkina Faso, Burundi, Cabo Verde, Cameroon, Central African Republic, Chad, Comoros, Congo, Democratic Republic of the Congo, Côte d'Ivoire, Djibouti, Egypt, Equatorial Guinea, Eritrea, Eswatini (Swaziland), Ethiopia, Gabon, Gambia, Ghana, Guinea, Guinea-Bissau, Kenya, Lesotho, Liberia, Libya, Madagascar, Malawi, Mali, Mauritania, Mauritius, Morocco, Mozambique, Namibia, Niger, Nigeria, Rwanda, São Tomé and Príncipe, Senegal, Seychelles, Sierra Leone, Somalia, South Africa, South Sudan, Sudan, Tanzania, Togo, Tunisia, Uganda, Zambia, Zimbabwe

**North America**: Antigua and Barbuda, Bahamas, Barbados, Belize, Canada, Costa Rica, Cuba, Dominica, Dominican Republic, El Salvador, Grenada, Guatemala, Haiti, Honduras, Jamaica, Mexico, Nicaragua, Panama, Saint Kitts and Nevis, Saint Lucia, Saint Vincent and the Grenadines, Trinidad and Tobago, United States (USA)

**South America**: Argentina, Bolivia, Brazil, Chile, Colombia, Ecuador, Guyana, Paraguay, Peru, Suriname, Uruguay, Venezuela

**Oceania**: Australia, Fiji, Kiribati, Marshall Islands, Micronesia, Nauru, New Zealand, Palau, Papua New Guinea, Samoa, Solomon Islands, Tonga, Tuvalu, Vanuatu

### Common Aliases Supported

- USA, United States
- UK, United Kingdom
- UAE, United Arab Emirates
- Czechia, Czech Republic
- Burma, Myanmar
- Swaziland, Eswatini
- Korea, South Korea
- Vatican, Vatican City
- East Timor, Timor-Leste
- Ivory Coast, Côte d'Ivoire
- Cape Verde, Cabo Verde
- And many more!

## Technologies Used

- **Rust** - Programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
