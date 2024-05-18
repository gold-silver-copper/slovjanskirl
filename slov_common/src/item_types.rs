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

#[derive(Clone, Debug, Display, PartialEq, EnumCount)]
pub enum AnimalType {
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType),
}

impl AnimalType {
    pub fn random_animaltype(small_rngik: &mut SmallRng) -> AnimalType {
        
        let y: f64 = small_rngik.gen(); // generates a float between 0 and 1

        if y < 0.5 { AnimalType::Mammal(MammalType::random_mammal_type(small_rngik)) }
        else if y < 0.8 {AnimalType::Bird(BirdType::random_bird_type(small_rngik))}
        else  {AnimalType::Lizard(LizardType::random_lizard_type(small_rngik))}


        




    }
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
pub struct Animal {
    pub animal_type: AnimalType,
    pub cur_health: HealthComponent,
    pub max_health: HealthComponent,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Human {
    pub inventory: InventoryComponent,
    pub equipment: EquipmentComponent,
    pub cur_health: HealthComponent,
    pub max_health: HealthComponent,
    pub name: NameComponent,
    pub stats: StatsComponent,
 
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Human(Human),
    Item(ItemType), //věć
    Monster(Animal),
    Mebelj(Mebelj),

    Råstlina(PlantType),

    None,
}

impl EntityType {
    pub fn minimal_string(&self) -> String {
        match self {
            EntityType::Human(x) => x.name.name.clone(),
            EntityType::Item(x) => x.minimal_string(),
            EntityType::Monster(x) => x.animal_type.minimal_string(),
            EntityType::Mebelj(x) => x.minimal_string(),
            EntityType::Råstlina(x) => x.minimal_string(),
            EntityType::None => String::new(),
        }
    }
    pub fn random_animal(small_rngik: &mut SmallRng) -> EntityType {

        EntityType::Monster(Animal { animal_type: AnimalType::random_animaltype(small_rngik), cur_health: HealthComponent::new(), max_health: HealthComponent::new() })
       
       





    }

  
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
    pub mebelj_type: MebeljType,
    pub material: SolidMaterial,
}
