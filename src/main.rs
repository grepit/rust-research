extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate percent_encoding;
extern crate anyhow;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize, Deserializer};
use std::str::FromStr;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    place_id: i64,
    osm_type: String,
    osm_id: i64,
    boundingbox: Vec<String>,
    #[serde(deserialize_with = "deserialize_float_from_str")]
    lat: f64,
    #[serde(deserialize_with = "deserialize_float_from_str")]
    lon: f64,
    display_name: String,
    class: String,
    type_: Option<String>, // Make 'type_' field optional
    importance: f64,
}

fn deserialize_float_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    f64::from_str(&s).map_err(serde::de::Error::custom)
}

#[tokio::main]
async fn main() -> Result<()> {
    let query = "4409 217th PL SE, Bothell WA 98021";
    let encoded_query = utf8_percent_encode(query, NON_ALPHANUMERIC).to_string();
    let endpoint = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json",
        encoded_query
    );

    let client = reqwest::Client::builder()
        .user_agent("Your User Agent Name")
        .build()?;
    let response = client.get(&endpoint).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let result: Vec<SearchResult> = serde_json::from_str(&body)
            .map_err(|e| anyhow!("Failed to parse JSON response: {}", e))?;

        if !result.is_empty() {
            println!("Results for query '{}':", query);
            for place in result {
                println!("  Place Name: {}", place.display_name);
                println!("  Type: {:?}", place.type_);
                println!("  Latitude: {}", place.lat);
                println!("  Longitude: {}\n", place.lon);
            }
        } else {
            println!("No results found for query '{}'", query);
        }
    } else {
        return Err(anyhow!("Error: Request failed with status code {}", response.status()));
    }

    Ok(())
}
