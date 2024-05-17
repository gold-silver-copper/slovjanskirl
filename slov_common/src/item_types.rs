use std::fmt::format;

use crate::*;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum SolidMaterial {
    Drěvo(WoodType),
    Metal(MetalType),
    Kamenj(StoneType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum FabricMaterial {
    //vlakno vivsa tkanina plet'
    Koža(MammalType),
    Tkanina(PlantType),

    Lancuh(MetalType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum PlantType {
    Drěvo(WoodType),
    Trava(GrassType),
    Kust(BushType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum AnimalType {
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimalPart {
    pub animal_type: AnimalType,
    pub animal_part: AnimalPartType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeleeWeapon {
    pub weapon_type: MeleeWeaponType,
    pub material_type: SolidMaterial,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangedWeapon {
    pub weapon_type: RangedWeaponType,

    pub tetiva_material: FabricMaterial,
    pub rema_material: WoodType,

    pub ammo_material: SolidMaterial,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ItemType {
    Melee(MeleeWeapon),
    Ranged(RangedWeapon),
    Clothing(ClothingItem),

    None,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ClothingType {
    Head(HeadClothingType),
    Shoulder(ShoulderClothingType),
    Torso(TorsoClothingType),
    Legs(LegsClothingType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClothingItem {
    pub clothing_type: ClothingType,
    pub fabric_type: FabricMaterial,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Human {
    pub inventory: InventoryComponent,
    pub equipment: EquipmentComponent,
    pub current_stats: StatsComponent,
    pub max_stats: StatsComponent,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Human(Human),
    Item(ItemType), //věć
    Monster(AnimalType),
    Mebelj(Mebelj),

    Råstlina(PlantType),

    None,
}

impl EntityType {
    pub fn minimal_string(&self) -> String {
        match self {
            EntityType::Human(x) => x.current_stats.name.clone(),
            EntityType::Item(x) => x.minimal_string(),
            EntityType::Monster(x) => x.minimal_string(),
            EntityType::Mebelj(x) => x.minimal_string(),
            EntityType::Råstlina(x) => x.minimal_string(),
            EntityType::None => String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
    pub mebelj_type: MebeljType,
    pub material: SolidMaterial,
}
