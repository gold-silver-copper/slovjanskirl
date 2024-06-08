use crate::*;

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct AnimalCSV {
    pub id: String,
    pub isv: String,
    pub animal_type: String,
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
}
#[derive(Debug, Deserialize)]
pub struct ClothingCSV {
    pub id: String,
    pub isv: String,

    pub item_type: String,
    pub primary_material: String,

    pub symbol: String,
}
#[derive(Debug, Deserialize)]
pub struct FurnitureCSV {
    pub id: String,
    pub isv: String,

    pub item_type: String,
    pub primary_material: String,
    pub symbol: String,
    pub move_block: String,
    pub vision_block: String,
    pub container: String,
}
#[derive(Debug, Deserialize)]
pub struct MaterialCSV {
    pub id: String,
    pub isv: String,

    pub material_type: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct PlantCSV {
    pub id: String,
    pub isv: String,
    pub plant_type: String,
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct WeaponCSV {
    pub id: String,
    pub isv: String,
    pub item_type: String,
    pub symbol: String,
    pub primary_material: String,
    pub secondary_material: String,
    pub attack_types: String,
   
}






#[derive(Debug, Deserialize)]
pub struct CSVTypeStore {
    pub animals: Vec<AnimalCSV>,
    pub clothing: Vec<ClothingCSV>,
    pub furniture: Vec<FurnitureCSV>,
    pub materials: Vec<MaterialCSV>,
    pub plants: Vec<PlantCSV>,
    pub weapons: Vec<WeaponCSV>,
}

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let parts: Vec<u8> = s
        .split(',')
        .map(|part| part.trim().parse().unwrap_or(0))
        .collect();

    if parts.len() != 3 {
        return Err(serde::de::Error::custom("invalid color format"));
    }

    Ok(Color::Rgb(parts[0], parts[1], parts[2]))
}
