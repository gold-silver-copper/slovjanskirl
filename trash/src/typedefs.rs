use ratatui::text::Line;

use crate::*;

pub type EntityID = u64;

pub type Nominative = EntityID;
pub type Accusative = EntityID;
pub type Dative = EntityID;
pub type Instrumental = EntityID;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Locative {
    Cardinal(CardinalDirection),
    Entity(EntityID),
    Point(MyPoint),
}

pub type PlayerMessage = String;
pub type GraphicTriple = (String, Color, Color);

pub type Health = i64;
pub type CoordinateUnit = i64;
pub const LOCAL_RANGE: i64 = 4000;
pub type MyPoint = (CoordinateUnit, CoordinateUnit, CoordinateUnit);
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CardinalDirection {
    North,
    West,
    South,
    East,
}

impl CardinalDirection {
    pub fn to_xyz(&self) -> MyPoint {
        match self {
            CardinalDirection::North => (0, 1, 0),
            CardinalDirection::West => (-1, 0, 0),
            CardinalDirection::South => (0, -1, 0),
            CardinalDirection::East => (1, 0, 0),
        }
    }
}

pub fn add_two_points(p1: &MyPoint, p2: &MyPoint) -> MyPoint {
    let mut result = (0, 0, 0);
    result.0 = p1.0 + p2.0;
    result.1 = p1.1 + p2.1;
    result.2 = p1.2 + p2.2;
    result
}

///NOTICE THIS ARRAY IS INDEXED Y FIRST FOR PERFORMANCE
pub fn create_2d_array(render_width: usize, render_height: usize) -> Vec<Vec<GraphicTriple>> {
    let grid = vec![vec![(" ".into(), Color::White, Color::Black); render_width]; render_height];
    grid
}

pub fn locate_square(e_pos: &MyPoint, w_radius: i64, h_radius: i64, z_shift: i64) -> AABB<MyPoint> {
    AABB::from_corners(
        (e_pos.0 - w_radius, e_pos.1 - h_radius, e_pos.2 + z_shift),
        (e_pos.0 + w_radius, e_pos.1 + h_radius, e_pos.2 + z_shift),
    )
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameDataPacket {
    pub entity_info: Vec<EntityPacket>,
    pub voxel_diffs: Vec<Voxel>,
    pub action_info: Vec<ActionPacket>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityPacket {
    pub entity_pos: MyPoint,
    pub entity_type: EntityType,
    pub entity_id: EntityID,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterData {
    pub player_id: EntityID,
    pub player_position: MyPoint,
}
#[derive(Debug)]
pub struct RenderPacket {
    pub spans_to_render: Vec<Vec<GraphicTriple>>,

    pub messages_to_render: Vec<PlayerMessage>,
}

impl RenderPacket {
    pub fn new() -> Self {
        RenderPacket {
            spans_to_render: Vec::new(),

            messages_to_render: Vec::new(),
        }
    }
}
