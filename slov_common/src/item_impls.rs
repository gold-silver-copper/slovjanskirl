use crate::*;

impl SolidMaterial {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Metal(x) => x.to_color(),
            Self::Drěvo(x) => x.to_color(),
            Self::Kamenj(x) => x.to_color(),
        }
    }
}



impl FabricMaterial {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Tkanina(x) => x.to_color(),
           Self::Koža(x) => x.to_color(),
            Self::Vlåkno(x) => x.to_color(),
            Self::Vivša(x) => x.to_color(),
            Self::Lancuh(x) => x.to_color(),
        }
    }
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


impl BushType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(228, 46, 103),
        }
    }
}



impl MammalType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(210, 180, 140),
        }
    }
}


impl FishType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(102, 205, 170),
        }
    }
}



impl BirdType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(128, 128, 0),
        }
    }
}


impl LizardType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(0, 128, 128),
        }
    }
}




impl AnimalType {
    pub fn to_displaychar(&self) -> String {
        let item_str = match &self {
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
        match &self {
            AnimalType::Bird(x) => x.to_color(),
            AnimalType::Mammal(x) => x.to_color(),
            AnimalType::Lizard(x) => x.to_color(),
            AnimalType::Fish(x) => x.to_color(),
        }
    }
}

impl RangedWeapon {
    pub fn to_color(&self) -> Color {
        match &self.weapon_type {
            _ => self.tetiva_material.to_color(),
          
        }
    }
}



impl ItemType {
    pub fn to_displaychar(&self) -> String {
        let item_str = self.to_title();

        // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_lowercase().to_string();
        ch
    }
    pub fn to_color(&self) -> Color {
        match &self {
            ItemType::Melee(x) => x.material_type.to_color(),
            ItemType::Ranged(x) => x.tetiva_material.to_color(),
            ItemType::Clothing(x) => x.fabric_type.to_color(),

            ItemType::None => Color::LightRed,
        }
    }
    pub fn to_title(&self) -> String {
        let item_str = match &self {
            ItemType::Melee(x) => {
                format!("{}", &x.weapon_type)
            }
            ItemType::Clothing(x) => {
                format!("{}", &x.clothing_type)
            }
            ItemType::Ranged(x) => {
                format!("{}", &x.weapon_type)
            }

            ItemType::None => "?".into(),
        };

        item_str
    }
}


impl Default for Human {
    fn default() -> Self {
        Self {
            inventory: Vec::new(),
            equipment: EquipmentComponent::new_empty(),
            current_stats: StatsComponent::new(&100),
            max_stats: StatsComponent::new(&100),
        }
    }
}


impl EntityType {
    pub fn to_displaychar(&self) -> String {
        match self {
            EntityType::Item(x) => x.to_displaychar(),
            EntityType::Monster(x) => x.to_displaychar(), //x.to_displaychar(),
            EntityType::Human(_) => "@".into(),
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
            EntityType::Human(_) => Color::White,
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

