use crate::*;



#[derive(Clone, Debug, Display, PartialEq)]
pub enum SolidMaterial {
    Drěvo(WoodType),
    Metal(MetalType),
    Kamenj(StoneType),
}


#[derive(Clone, Debug, Display, PartialEq)]
pub enum FabricMaterial {
    Vlåkno(WoodType),
    Koža(MammalType),
    Tkanina(PlantType),
    Vivša(BushType),
    Lancuh(MetalType),
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
pub struct Animal {
    pub animal_type: AnimalType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeleeWeapon {
    pub weapon_type: MeleeWeaponType,
    pub material_type: SolidMaterial,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangedWeapon {
    pub weapon_type: RangedWeaponType,
    pub tulec_material: FabricMaterial,
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

#[derive(Clone, Debug, PartialEq)]
pub struct ClothingItem {
    pub clothing_type: ClothingItemType,
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
    Monster(Animal),
    Mebelj(Mebelj),
    Drěvo(WoodType),
    Råstlina(PlantType),
    Kust(BushType),
    //Statuja(AnimalType),
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
   pub mebelj_type: MebeljType,
   pub material: SolidMaterial,
}

