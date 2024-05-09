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
pub enum RangedWeaponType {
    Lųk(Bow),
    Proca(Sling),
    Prak(Sling),
    Prašča(Sling),
    Šlojder(Sling),
    Kuša(CrossBow),
    Samostrěl(CrossBow),
    Arbalet(CrossBow)
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
    Gųsę
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
    pub weapon_type: MeleeWeaponType,
    pub material_type: Material,
}

#[derive(Clone, Debug)]
pub struct RangedWeapon {
    pub weapon_type: RangedWeaponType,
    
}

#[derive(Clone, Debug)]
pub struct Bow {
    pub rame_luka: WoodType,
    pub tetiva: Fabric,
}

#[derive(Clone, Debug)]
pub struct CrossBow {
    pub luk: Bow,
    pub telo: Material,
}

#[derive(Clone, Debug)]
pub struct Sling {
    pub material_type: Fabric,
}
#[derive(Clone, Debug, Display)]
pub enum ItemType {
    Melee(MeleeWeapon),
    Ranged(RangedWeapon),
    Ammo(Ammo),
}

#[derive(Clone, Debug)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn to_char(&self) -> String {

        let item_str = match &self.item_type {
            ItemType::Melee(x) => {format!("{}", &x.weapon_type)},
            ItemType::Ranged(x) => {format!("{}", &x.weapon_type)},
            ItemType::Ammo(x) => {format!("{}", &x.ammo_type)},

        };

       // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_lowercase().to_string();
        ch
    }
}

#[derive(Clone, Debug)]
pub struct MyEntity {
    pub position_component: PositionComponent,
    pub entity_type: EntityType,
}
#[derive(Clone, Debug)]
pub struct Player {
    inventory: InventoryComponent,
    health: HealthComponent,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: Vec::new(),
            health: HealthComponent::new(&100),
        }
    }
}

#[derive(Clone, Debug, Display)]
pub enum EntityType {
    Player(Player),
    Item(Item),
    Monster(Animal),
}

impl EntityType {
    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = match self {
            EntityType::Item(x) => x.to_char(),
            EntityType::Monster(_) => "M".into(),
            EntityType::Player(_) => "@".into(),
        };
        (ent_char.into(), Color::Red, Color::Black)
    }
}

#[derive(Clone, Debug)]
pub struct Furniture {
    furniture_type: FurnitureType,
    material_type: Material,
}
