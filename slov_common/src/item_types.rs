use crate::*;
#[derive(Clone, Debug, Display, PartialEq)]
pub enum MeleeWeaponType {
    Nož,
    Sěkyra,
    Kyj,
    Meč,
    Kopje,
   // Nagajka,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum RangedWeaponType {
    Lųk,
  //  Proca(Sling),
   // Prak(Sling),
    Prašča,
  //  Šlojder(Sling),
 //   Kuša(CrossBow),
    Samostrěl,
    Kameni,
 //   Arbalet(CrossBow),
}



#[derive(Clone, Debug, Display, PartialEq)]
pub enum Material {
    Drěvo(WoodType),
    Metal(MetalType),
    Kamenj(StoneType),
}

impl Material {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Metal(x) => x.to_color(),
            Self::Drěvo(x) => x.to_color(),
            Self::Kamenj(x) => x.to_color(),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Fabric {
    Pulpa(WoodType),
    Vlås(MammalType),
    Koža(MammalType),
    Tkanina(PlantType),
    Plåtno(BushType),
}

impl Fabric {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Tkanina(x) => x.to_color(),
            Self::Vlås(x) | Self::Koža(x) => x.to_color(),
            Self::Pulpa(x) => x.to_color(),
            Self::Plåtno(x) => x.to_color(),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum StoneType {
    Granit,
    Kremenj,
    Rubin,
    Mramor,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum MetalType {
    Bronza,
    Zlåto,
    Železo,
    Srebro,
    Medj,
}

#[derive(Clone, Debug, Display, PartialEq)]
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

#[derive(Clone, Debug, Display, PartialEq)]
pub enum PlantType {
    Trava,
    Kovylj, //needle grass
    Burjan, // high grass
    Kanabis,
    Jasenėc,
}
impl PlantType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(34, 139, 34),
        }
    }
    pub fn to_displaychar(&self) -> String {
        match &self {
            Self::Trava => "'".into(),
            Self::Kovylj => "\"".into(),
            Self::Burjan => "/".into(),
            Self::Kanabis => "\"".into(),
            Self::Jasenėc => "\"".into(),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum BushType {
    Klubnika,
    Jagoda,     //needle grass
    Zemljanika, // high grass
    Ježina,
    Kųpina,
    Brusnica,
    Malina,
    Kljukva,
    Črnica,
    Žuravina,
    Bȯzina,
}
impl BushType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(228, 46, 103),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum AnimalType {
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType),
}

#[derive(Clone, Debug, Display, PartialEq)]
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

#[derive(Clone, Debug, Display, PartialEq)]
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

#[derive(Clone, Debug, Display, PartialEq)]
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

#[derive(Clone, Debug, Display, PartialEq)]
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

#[derive(Clone, Debug, Display, PartialEq)]
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

#[derive(Clone, Debug, Display, PartialEq)]
pub enum AmmoType {
    Kulja,
    Strěla,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimalPart {
    pub animal_type: AnimalType,
    pub animal_part: AnimalPartType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Animal {
    pub animal_type: AnimalType,
}

impl Animal {
    pub fn to_displaychar(&self) -> String {
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


#[derive(Clone, Debug, PartialEq)]
pub struct MeleeWeapon {
    pub weapon_type: MeleeWeaponType,
    pub material_type: Material,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangedWeapon {
    pub weapon_type: RangedWeaponType,
    pub tulec_material: Fabric, 
    pub tetiva_material: Fabric, 
    pub rema_material: Material,

    pub ammo_material: Material,


}

impl RangedWeapon {
    pub fn to_color(&self) -> Color {
        match &self.weapon_type {
            RangedWeaponType::Samostrěl => self.tulec_material.to_color(),
            RangedWeaponType::Prašča => {
                self.tulec_material.to_color()
            }
            RangedWeaponType::Lųk => self.tulec_material.to_color(),
            RangedWeaponType::Kameni => self.tulec_material.to_color(),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ItemType {
    Melee(MeleeWeapon),
    Ranged(RangedWeapon),
    
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn to_displaychar(&self) -> String {
        let item_str = self.to_title();

        // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_lowercase().to_string();
        ch
    }
    pub fn to_color(&self) -> Color {
        match &self.item_type {
            ItemType::Melee(x) => x.material_type.to_color(),
            ItemType::Ranged(x) => x.tulec_material.to_color(),
         
            ItemType::None => Color::LightRed,
        }
    }
    pub fn to_title(&self) -> String {
        let item_str = match &self.item_type {
            ItemType::Melee(x) => {
                format!("{}", &x.weapon_type)
            }
            ItemType::Ranged(x) => {
                format!("{}", &x.weapon_type)
            }
         
            ItemType::None => "?".into(),
        };

        item_str
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub inventory: InventoryComponent,
    pub health: HealthComponent,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: Vec::new(),
            health: HealthComponent::new(&100),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Player(Player),
    Item(Item), //věć
    Monster(Animal),
    Mebelj(Mebelj),
    Drěvo(WoodType),
    Råstlina(PlantType),
    Kust(BushType),
    //Statuja(AnimalType),
    
    None,
}


impl EntityType {
    pub fn to_displaychar(&self) -> String {
        match self {
            EntityType::Item(x) => x.to_displaychar(),
            EntityType::Monster(x) => x.to_displaychar(), //x.to_displaychar(),
            EntityType::Player(_) => "@".into(),
            EntityType::None => "?".into(),
            EntityType::Drěvo(x) => "t".into(),
            EntityType::Kust(x) => "*".into(),
            EntityType::Råstlina(x) => x.to_displaychar(),
            EntityType::Mebelj(x) => x.to_displaychar(),
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            EntityType::Item(x) => x.to_color(),
            EntityType::Monster(x) => x.to_color(),
            EntityType::Player(_) => Color::Red,
            EntityType::None => Color::Red,
            EntityType::Drěvo(x) => x.to_color(),
            EntityType::Kust(x) => x.to_color(),
            EntityType::Mebelj(x) => x.to_color(),
            EntityType::Råstlina(x) => x.to_color(),
        }
    }

    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = self.to_displaychar();
        let ent_color = self.to_color();
        (ent_char, ent_color, Color::Black)
    }
}


#[derive(Clone, Debug, Display, PartialEq)]
pub enum MebeljType {
    Stěna,
    Stol,
    Stul,
    Skrinja, //sunduk
    Dvėrj,
    Vråta,
    Vaza,
    Škaf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
    mebelj_type: MebeljType,
    material: Material,
}

impl Mebelj {
    pub fn to_displaychar(&self) -> String {
        match &self.mebelj_type {
            MebeljType::Stěna => "#".into(),
            MebeljType::Dvėrj => "+".into(),
            MebeljType::Vråta => "=".into(),
            _ => todo!("implement mebelj"),
        }
    }
    pub fn to_color(&self) -> Color {
        self.material.to_color()
    }
}








/*
#[derive(Clone, Debug, PartialEq)]
pub struct Bow {
    pub rame_luka: WoodType,
    pub tetiva: Fabric,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CrossBow {
    pub luk: Bow,
    pub telo: Material,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sling {
    pub material_type: Fabric,
} */