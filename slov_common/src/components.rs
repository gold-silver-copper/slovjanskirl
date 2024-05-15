use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PositionComponent {
    pub entity_id: EntityID,
    pub point: MyPoint,
}

impl RTreeObject for PositionComponent {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.point.0, self.point.1))
    }
}

impl PointDistance for PositionComponent {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.point.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.point.contains_point(point)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatsComponent {
    pub name: String,
    pub health: StatsUnit,       //zdravje
    pub stamina_air: StatsUnit,  //vozduh
    pub sila: StatsUnit,     //sila
    pub bystrost: StatsUnit,      // bystrost
    pub razum: StatsUnit, //razum

   
}

impl StatsComponent {
    pub fn new_default() -> StatsComponent {
        StatsComponent {
            name: String::from("zlotik"),
            health: 100,
            stamina_air: 100,
            sila: 100,
            bystrost: 100,
            razum: 100,
            
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentComponent {
    pub melee_weapon: Option<MeleeWeapon>,

    pub ranged_weapon: Option<RangedWeapon>,

    pub head: Option<ClothingItem>,
    pub shoulders: Option<ClothingItem>,
    pub torso: Option<ClothingItem>,
    pub legs: Option<ClothingItem>,
}

impl EquipmentComponent {
    pub fn new_empty() -> EquipmentComponent {
        EquipmentComponent {
            melee_weapon: None,

            ranged_weapon: None,

            head: None,
            shoulders: None,
            torso: None,
            legs: None,

        }
    }
}

pub type InventoryComponent = Vec<ItemType>;
