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

pub fn load_csv_data() -> CSVTypeStore {

    let data_csv = include_bytes!("../../assets/data/isv_animals.csv");
   

    let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());

    let mut animals_vec = Vec::new();
    for data_item in csv_reader.deserialize() {
        let data_item: AnimalCSV = data_item.unwrap();

        animals_vec.push(data_item);
    }


    let data_csv = include_bytes!("../../assets/data/isv_clothing.csv");
   

    let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());

    let  mut clothing_vec = Vec::new();
    for data_item in csv_reader.deserialize() {
        let data_item: ClothingCSV = data_item.unwrap();

        clothing_vec.push(data_item);
    }

    let data_csv = include_bytes!("../../assets/data/isv_furniture.csv");
   

    let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());

    let  mut furniture_vec = Vec::new();
    for data_item in csv_reader.deserialize() {
        let data_item: FurnitureCSV = data_item.unwrap();

        furniture_vec.push(data_item);
    }

    let data_csv = include_bytes!("../../assets/data/isv_materials.csv");
   

    let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());

    let mut  material_vec = Vec::new();
    for data_item in csv_reader.deserialize() {
        let data_item: MaterialCSV = data_item.unwrap();

        material_vec.push(data_item);
    }
    
    let data_csv = include_bytes!("../../assets/data/isv_plants.csv");
   

    let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());

    let  mut plant_vec = Vec::new();
    for data_item in csv_reader.deserialize() {
        let data_item: PlantCSV = data_item.unwrap();

        plant_vec.push(data_item);
    }

    let data_csv = include_bytes!("../../assets/data/isv_weapons.csv");
   

    let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());

    let  mut weapon_vec = Vec::new();
    for data_item in csv_reader.deserialize() {
        let data_item: WeaponCSV = data_item.unwrap();

        weapon_vec.push(data_item);
    }

    CSVTypeStore{
        animals: animals_vec,
        clothing: clothing_vec,
        furniture: furniture_vec,
        materials: material_vec,
        plants: plant_vec,
        weapons: weapon_vec







    }

}
