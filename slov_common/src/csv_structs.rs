use crate::*;


use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
pub struct AnimalCSV {
    id: String,
    isv: String,
    variations: String,
    animal_type: String,
    symbol: String,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
}


pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let parts: Vec<u8> = s.split(',')
        .map(|part| part.trim().parse().unwrap_or(0))
        .collect();

    if parts.len() != 3 {
        return Err(serde::de::Error::custom("invalid color format"));
    }

    Ok(Color::Rgb(parts[0],parts[1],parts[2]))
}