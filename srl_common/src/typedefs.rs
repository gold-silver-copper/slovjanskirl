use crate::*;

pub type EntityID = u64;
pub type AccountID = u64;
pub type Health = u64;
pub type CoordinateUnit = i64;
pub const LOCAL_RANGE: i64 = 4000;
pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (String, Color, Color);
pub type RoofType = ();
pub type FloorType = ();
pub type FurnitureType = ();
pub type Nominative = EntityID;
pub type Accusative = EntityID;
pub type Dative = EntityID;
pub type Instrumental = EntityID;

#[derive(Clone, Debug)]
pub enum EntityType {
    Player,
    Item,
    Monster,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Locative {
    Cardinal(CardinalDirection),
    Entity(EntityID),
    Point(MyPoint),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    West,
    South,
    East,
}

impl CardinalDirection {
    pub fn to_xyz(&self) -> MyPoint {
        match self {
            CardinalDirection::North => (0, 1),
            CardinalDirection::West => (-1, 0),
            CardinalDirection::South => (0, -1),
            CardinalDirection::East => (1, 0),
        }
    }
}
