use crate::*;
#[derive(Clone, Debug, Display,PartialEq)]
pub enum MeleeWeaponType {
    Nož,
    Sěkyra,
    Kyj,
    Meč,
    Kopje,
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum RangedWeaponType {
    Lųk(Bow),
    Proca(Sling),
    Prak(Sling),
    Prašča(Sling),
    Šlojder(Sling),
    Kuša(CrossBow),
    Samostrěl(CrossBow),
    Arbalet(CrossBow),
}

impl RangedWeaponType {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Arbalet(x) | Self::Kuša(x) | Self::Samostrěl(x) => x.telo.to_color(),
            Self::Šlojder(x) | Self::Prak(x) | Self::Prašča(x) | Self::Proca(x) => {
                x.material_type.to_color()
            }
            Self::Lųk(x) => x.rame_luka.to_color(),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum Material {
    Wood(WoodType),
    Metal(MetalType),
    Stone(StoneType),
}

impl Material {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Metal(x) => x.to_color(),
            Self::Stone(x) => x.to_color(),
            Self::Wood(x) => x.to_color(),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum Fabric {
    Pulp(WoodType),
    Hair(MammalType),
    Leather(MammalType),
    Cloth(PlantType),
}

impl Fabric {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Cloth(x) => x.to_color(),
            Self::Hair(x) | Self::Leather(x) => x.to_color(),
            Self::Pulp(x) => x.to_color(),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum StoneType {
    Granit,
    Kremenj,
    Rubin,
    Mramor,
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum MetalType {
    Bronza,
    Zlåto,
    Železo,
    Srebro,
    Medj,
}

#[derive(Clone, Debug, Display,PartialEq)]
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

impl WoodType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(139, 69, 19),
        }
    }
}

impl MetalType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(170, 169, 173),
        }
    }
}

impl StoneType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(118, 91, 70),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum FurnitureType {
    Stěna(Material),
    Stol(Material),
    Stul(Material),
    Skrinja(Material), //sunduk
    Dvėrj(Material),
    Vråta(Material),
    Vaza(Material),
    Škaf(Material),
    Drěvo(WoodType),
    Råstlina(PlantType),
    Statuja(AnimalType),
    Air

}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum PlantType {
    Trava,
    Kovylj, //needle grass
    Burjan, // high grass
    Kanabis,
    Jasenėc
}
impl PlantType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(34, 139, 34),
        }
    }
}
#[derive(Clone, Debug, Display,PartialEq)]
pub enum AnimalType {
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType),
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum MammalType {
    Los,
    Jelenj,
    Krava,
    Pes,
    Tigr,
}

impl MammalType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(210, 180, 140),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum FishType {
    Losos,
    Tunec,
    Karas,
}

impl FishType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(102, 205, 170),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum BirdType {
    Sova,
    Vrabec,
    Vran,
    Gavran,
    Kos,
    Gųsę,
}

impl BirdType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(128, 128, 0),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
pub enum LizardType {
    Gad,
    Jaščer,
    Iguana,
    Vųž,
}

impl LizardType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(0, 128, 128),
        }
    }
}

#[derive(Clone, Debug, Display,PartialEq)]
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

#[derive(Clone, Debug, Display,PartialEq)]
pub enum AmmoType {
    Kulja,
    Strěla,
}

#[derive(Clone, Debug,PartialEq)]
pub struct AnimalPart {
    animal_type: AnimalType,
    animal_part: AnimalPartType,
}

#[derive(Clone, Debug,PartialEq)]
pub struct Animal {
    animal_type: AnimalType,
}

impl Animal {
    pub fn to_char(&self) -> String {
        let item_str = match &self.animal_type {
            AnimalType::Bird(x) => {
                format!("{}", &x)
            }
            AnimalType::Mammal(x) => {
                format!("{}", &x)
            }
            AnimalType::Lizard(x) => {
                format!("{}", &x)
            }
            AnimalType::Fish(x) => {
                format!("{}", &x)
            }
        };

        // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_string();
        ch
    }
    pub fn to_color(&self) -> Color {
        match &self.animal_type {
            AnimalType::Bird(x) => x.to_color(),
            AnimalType::Mammal(x) => x.to_color(),
            AnimalType::Lizard(x) => x.to_color(),
            AnimalType::Fish(x) => x.to_color(),
        }
    }
}

#[derive(Clone, Debug,PartialEq)]
pub struct Ammo {
    ammo_type: AmmoType,
    material_type: Material,
    quantity: i64,
}

#[derive(Clone, Debug,PartialEq)]
pub struct MeleeWeapon {
    pub weapon_type: MeleeWeaponType,
    pub material_type: Material,
}

#[derive(Clone, Debug,PartialEq)]
pub struct RangedWeapon {
    pub weapon_type: RangedWeaponType,
}

#[derive(Clone, Debug,PartialEq)]
pub struct Bow {
    pub rame_luka: WoodType,
    pub tetiva: Fabric,
}

#[derive(Clone, Debug,PartialEq)]
pub struct CrossBow {
    pub luk: Bow,
    pub telo: Material,
}

#[derive(Clone, Debug,PartialEq)]
pub struct Sling {
    pub material_type: Fabric,
}
#[derive(Clone, Debug, Display,PartialEq)]
pub enum ItemType {
    Melee(MeleeWeapon),
    Ranged(RangedWeapon),
    Ammo(Ammo),
}

#[derive(Clone, Debug,PartialEq)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn to_char(&self) -> String {
        let item_str = match &self.item_type {
            ItemType::Melee(x) => {
                format!("{}", &x.weapon_type)
            }
            ItemType::Ranged(x) => {
                format!("{}", &x.weapon_type)
            }
            ItemType::Ammo(x) => {
                format!("{}", &x.ammo_type)
            }
        };

        // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_lowercase().to_string();
        ch
    }
    pub fn to_color(&self) -> Color {
        match &self.item_type {
            ItemType::Melee(x) => x.material_type.to_color(),
            ItemType::Ranged(x) => x.weapon_type.to_color(),
            ItemType::Ammo(x) => x.material_type.to_color(),
        }
    }
}

#[derive(Clone, Debug,PartialEq)]
pub struct MyEntity {
    pub position_component: PositionComponent,
    pub entity_type: EntityType,
}
#[derive(Clone, Debug,PartialEq)]
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

#[derive(Clone, Debug, Display,PartialEq)]
pub enum EntityType {
    Player(Player),
    Item(Item),
    Monster(Animal),
}

impl EntityType {
    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = match self {
            EntityType::Item(x) => x.to_char(),
            EntityType::Monster(x) => x.to_char(),
            EntityType::Player(_) => "@".into(),
        };
        let ent_color = match self {
            EntityType::Item(x) => x.to_color(),
            EntityType::Monster(x) => x.to_color(),
            EntityType::Player(_) => Color::Red,
        };
        (ent_char, ent_color, Color::Black)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Furniture {
   pub furniture_type: FurnitureType,
    
}
