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
    pub health: StatsUnit, //zdravje
    pub stamina_air: StatsUnit, //vozduh
    pub strength: StatsUnit, //sila
    pub agility: StatsUnit, // bystrost
    pub speed: StatsUnit, //szybkost
    pub intelligence: StatsUnit, //razum
  
   
    pub milost: StatsUnit, //cuteness

}

impl StatsComponent {
    pub fn new(max: &i64) -> StatsComponent {
        StatsComponent {
            health: max.clone(),
             stamina_air: max.clone(),
             strength: max.clone(),
             agility: max.clone(),
             speed: max.clone(),
             intelligence: max.clone(),
           
             milost: max.clone(),
          
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentComponent {
    pub melee_weapon: Option<MeleeWeapon>,

    pub ranged_weapon: Option<RangedWeapon>,

    pub head: Option<ClothingItem>,
    pub shawl: Option<ClothingItem>, //add style option so you can wear shawl in different ways
    pub body: Option<ClothingItem>,
    pub legs: Option<ClothingItem>,
    pub feet: Option<ClothingItem>,
    pub neck: Option<ClothingItem>, //našijnik ogrlica monisto
    pub finger: Option<ClothingItem>,


   

}

impl EquipmentComponent {
    pub fn new_empty() -> EquipmentComponent {
        EquipmentComponent {
             melee_weapon: None,

     ranged_weapon: None,

     head: None,
     shawl: None, //add style option so you can wear shawl in different ways
     body: None,
     legs: None,
     feet: None,
     neck: None, //našijnik ogrlica monisto
     finger: None,
          
          
        }
    }
}








pub type InventoryComponent = Vec<Item>;
