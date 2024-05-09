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
    Medj,
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
    Kaštan,
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


#[derive(Clone, Debug, Display)]
pub enum PlantType {
    Trava,
    Kovylj, //needle grass
    Burjan, // high grass
    Kanabis,
}
#[derive(Clone, Debug, Display)]
pub enum AnimalType {
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType),
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


#[derive(Clone, Debug, Display)]
pub enum AmmoType {
    Kulja,
    Strěla,
}

#[derive(Clone, Debug)]
pub struct AnimalPart {
    animal_type: AnimalType,
    animal_part: AnimalPartType,
}

#[derive(Clone, Debug)]
pub struct Animal {
    animal_type: AnimalType,
   
}

#[derive(Clone, Debug)]
pub struct Ammo {
    ammo_type: AmmoType,
    material_type: Material,
    quantity: i64,
}


#[derive(Clone, Debug)]
pub struct MeleeWeapon {
    weapon_type: MeleeWeaponType,
    material_type: Material,
}

#[derive(Clone, Debug)]
pub struct Bow {
    rame_luka: WoodType,
    tetiva: Fabric,
}

#[derive(Clone, Debug)]
pub struct Sling {
    material_type: Fabric,
}
#[derive(Clone, Debug, Display)]
pub enum ItemType {

    Melee(MeleeWeapon),
    Bow(Bow),
    Sling(Sling),
    Ammo(Ammo),

}


#[derive(Clone, Debug)]
pub struct Item {
    item_type: ItemType,
    
}

#[derive(Clone, Debug)]
pub struct MyEntity {
  pub  position_component: PositionComponent,
   pub entity_type: EntityType,

}
#[derive(Clone, Debug)]
pub struct Player {
    inventory: Vec<Item>,

}
impl Default for Player {
    fn default() -> Self {
        Self{
            inventory: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EntityType {
    Player(Player),
    Item(Item),
    Monster(AnimalType),
}

impl EntityType {
    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = match self {
            EntityType::Item(_) => "i",
            EntityType::Monster(_) => "M",
            EntityType::Player(_) => "@",
        };
        (ent_char.into(), Color::Red, Color::Black)
    }
}


#[derive(Clone, Debug)]
pub struct Furniture {
    furniture_type: FurnitureType,
    material_type: Material,
}