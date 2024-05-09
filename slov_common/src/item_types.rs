use crate::*;
#[derive(Clone, Debug, Display)]
pub enum MeleeWeaponType {
    Nož,
    Sěkyra,
    Kyj,
    Meč,
    Kopje,

}


#[derive(Clone, Debug, Display)]
pub enum Material {
    Wood(WoodType),
    Metal(MetalType),
    Stone(StoneType),
}

#[derive(Clone, Debug, Display)]
pub enum Fabric {
    Pulp(WoodType),
    Hair(MammalType),
    Leather(MammalType),
    Cloth(PlantType),
}

#[derive(Clone, Debug, Display)]
pub enum StoneType {
    Granit,
    Kremenj,
    Rubin,
    Mramor,
}

#[derive(Clone, Debug, Display)]
pub enum MetalType {
    Bronza,
    Zlåto,
    Železo,
    Srebro,
    Medj
}

#[derive(Clone, Debug, Display)]
pub enum WoodType {
    Brest,
    Jasenj,
    Lipa,
    Jablanj,
    Kalina,
    Jalovec,
    Brek,
    Kaštan
}


#[derive(Clone, Debug)]
pub struct MeleeWeapon {
    weapon_type : MeleeWeaponType,
    material_type: Material
}


#[derive(Clone, Debug)]
pub struct Bow {
    rame_luka : WoodType,
    tetiva: Fabric
}

#[derive(Clone, Debug)]
pub struct Sling {
    material_type: Fabric,
}


#[derive(Clone, Debug, Display)]
pub enum FurnitureType {
    Stěna,
    Stol,
    Stul,
    Skrinja, //sunduk
    Dvėrj,
    Vråta,
    Vaza,
    Škaf,

   
}

#[derive(Clone, Debug)]
pub struct Furniture {
    furniture_type: FurnitureType,
    material_type: Material
}

#[derive(Clone, Debug, Display)]
pub enum PlantType {
   Trava,
   Kovylj, //needle grass
   Burjan, // high grass
   Kanabis,



   
}
#[derive(Clone, Debug, Display)]
pub enum AnimalType{ 
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType)

}

#[derive(Clone, Debug, Display)]
pub enum MammalType {
   Los,
   Jelenj,
   Krava,
   Pes,
   Tigr,
   
}

#[derive(Clone, Debug, Display)]
pub enum FishType {
   Losos,
   Tunec,
   Karas,
   
}

#[derive(Clone, Debug, Display)]
pub enum BirdType {
   Sova,
   Vrabec,
   Vran,
   Gavran,
   Kos,
   
}

#[derive(Clone, Debug, Display)]
pub enum LizardType {
   Gad,
   Jaščer,
   Iguana,
   Vųž,
   
}

#[derive(Clone, Debug, Display)]
pub enum AnimalPartType {
   Head,
   Tail,
   Body,
   Leg,
   Feather,
   Skin,
   Hair,
   Breast,
   Bone,
   
}

#[derive(Clone, Debug)]
pub struct AnimalPart {
    animal_type : AnimalType,
    animal_part: AnimalPartType
}


#[derive(Clone, Debug)]
pub struct Ammo {
    ammo_type : AmmoType,
    material_type: Material,
    quantity: i64
}


#[derive(Clone, Debug, Display)]
pub enum AmmoType {
   Kulja,
   Strěla
   
}
