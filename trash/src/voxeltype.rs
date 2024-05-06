use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum VoxelType {
    Gas(GasType),
    Liquid(LiquidType),
    Solid(SolidType),
    Furniture(FurnitureType),
    Plant(PlantType),
}
impl VoxelType {
    pub fn grapheme(&self) -> String {
        match self {
            VoxelType::Gas(_) => "჻".into(), // Georgian letter, symbolizing something ethereal or gaseous
            VoxelType::Liquid(_) => "≈".into(), // Wavy lines, representing liquid
            VoxelType::Solid(_) => "▓".into(), // Dense block, symbolizing solid matter
            VoxelType::Furniture(_) => "⌂".into(), // House symbol, indicating furniture or man-made objects
            VoxelType::Plant(_) => "♣".into(), // Club symbol, often used to represent vegetation
        }
    }
    pub fn color(&self) -> Color {
        match self {
            VoxelType::Gas(gas_type) => gas_type.color(),
            VoxelType::Liquid(liquid_type) => liquid_type.color(),
            VoxelType::Solid(solid_type) => match solid_type {
                SolidType::Metal(metal_type) => metal_type.color(),
                SolidType::Stone(stone_type) => stone_type.color(),
                SolidType::Wood(wood_type) => wood_type.color(),
                SolidType::Frozen(liquid_type) => liquid_type.color(),
                SolidType::Granular(granular_type) => granular_type.color(),
            },
            VoxelType::Furniture(furniture_type) => furniture_type.color(),
            VoxelType::Plant(plant_type) => plant_type.color(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GasType {
    Air,
    Smoke,
}

impl GasType {
    pub fn color(&self) -> Color {
        match self {
            GasType::Air => Color::LightCyan, // Light Cyan to represent air, which is often visually represented as clear
            GasType::Smoke => Color::DarkGray, // Dark Gray for smoke, as it's typically seen as a dark cloud
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FurnitureType {
    Chair(SolidType),
    Table(SolidType),
    Bed(SolidType),
}

impl FurnitureType {
    pub fn color(&self) -> Color {
        match self {
            FurnitureType::Chair(solty) => solty.color(), // Light Cyan to represent air, which is often visually represented as clear
            FurnitureType::Bed(solty) => solty.color(),
            FurnitureType::Table(solty) => solty.color(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FloorType {
    Raw(SolidType),
    Flooring(SolidType),
    Air,
}

impl FloorType {
    pub fn color(&self) -> Color {
        match self {
            Self::Raw(st) => st.color(),
            Self::Flooring(st) => st.color(),
            Self::Air => Color::Black,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SolidType {
    Metal(MetalType),
    Stone(StoneType),
    Wood(WoodType),
    Frozen(LiquidType),
    Granular(GranularType),
}

impl SolidType {
    pub fn color(&self) -> Color {
        match self {
            SolidType::Metal(mt) => mt.color(),
            SolidType::Stone(st) => st.color(),
            SolidType::Wood(wt) => wt.color(),
            SolidType::Frozen(lt) => lt.color(),
            SolidType::Granular(gt) => gt.color(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]

// Define MetalType pub enum for various types of metals
pub enum MetalType {
    Iron,
    Gold,
    Silver,
    Copper,
    Steel,
    Aluminum,
    Titanium,
}

impl MetalType {
    pub fn color(&self) -> Color {
        match self {
            MetalType::Iron => Color::Gray,
            MetalType::Gold => Color::Yellow,
            MetalType::Silver => Color::White,
            MetalType::Copper => Color::Red,
            MetalType::Steel => Color::DarkGray,
            MetalType::Aluminum => Color::LightCyan,
            MetalType::Titanium => Color::White,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]

// Define StoneType pub enum for various types of stones
pub enum StoneType {
    Granite,
    Marble,
    Limestone,
    Slate,
    Sandstone,
    Quartz,
}

impl StoneType {
    pub fn color(&self) -> Color {
        match self {
            StoneType::Granite => Color::DarkGray,
            StoneType::Marble => Color::White,
            StoneType::Limestone => Color::LightYellow,
            StoneType::Slate => Color::Gray,
            StoneType::Sandstone => Color::LightRed,
            StoneType::Quartz => Color::LightCyan,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]

// Define WoodType pub enum for various types of wood
pub enum WoodType {
    Oak,
    Pine,
    Birch,
    Maple,
    Cherry,
    Walnut,
}

impl WoodType {
    pub fn color(&self) -> Color {
        match self {
            WoodType::Oak => Color::Yellow,
            WoodType::Pine => Color::LightGreen,
            WoodType::Birch => Color::White,
            WoodType::Maple => Color::LightRed,
            WoodType::Cherry => Color::Red,
            WoodType::Walnut => Color::DarkGray,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]

// Reusing LiquidType pub enum for the Frozen variant in SolidType
pub enum LiquidType {
    Water,
    Beer,
    Lava,
    Oil,
    Acid,
}

impl LiquidType {
    pub fn color(&self) -> Color {
        match self {
            LiquidType::Water => Color::Blue,
            LiquidType::Beer => Color::LightYellow,
            LiquidType::Lava => Color::Red,
            LiquidType::Oil => Color::DarkGray,
            LiquidType::Acid => Color::LightGreen,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// Define GranularType pub enum for various granular materials
pub enum GranularType {
    Sand,
    Gravel,
    Salt,
    Sugar,
    Dirt,
}

impl GranularType {
    pub fn color(&self) -> Color {
        match self {
            GranularType::Sand => Color::Yellow,
            GranularType::Gravel => Color::Gray,
            GranularType::Salt => Color::White,
            GranularType::Sugar => Color::LightYellow,
            GranularType::Dirt => Color::Green,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FlowerType {
    Rose,
    Tulip,
    Daisy,
    Sunflower,
    Lavender,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BushType {
    Berry(BerryType),
    Shrub,
    Hedge,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BerryType {
    Raspberry,
    Blueberry,
    Blackberry,
    Strawberry,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GrassType {
    TallGrass,
    ShortGrass,
    WheatGrass,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AquaticPlantType {
    Seaweed,
    WaterLily,
    Lotus,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CropType {
    Wheat,
    Corn,
    Rice,
    Tomato,
    Potato,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PlantType {
    Tree(WoodType),
    Flower(FlowerType),
    Bush(BushType),
    Grass(GrassType),
    Aquatic(AquaticPlantType),
    Crop(CropType),
}

impl PlantType {
    pub fn color(&self) -> Color {
        match self {
            _ => Color::Green,
        }
    }
}
