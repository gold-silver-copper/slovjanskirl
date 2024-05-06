use crate::*;

pub type EntityID = u64;
pub type AccountID = u64;
pub type Health = u64;
pub type CoordinateUnit = i64;
pub const LOCAL_RANGE: i64 = 4000;
pub type MyPoint = (CoordinateUnit, CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (String, Color, Color);
pub type RoofType = ();
pub type FloorType = ();
pub type FurnitureType = ();
pub type Nominative = EntityID;
pub type Accusative = EntityID;
pub type Dative = EntityID;
pub type Instrumental = EntityID;
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
            CardinalDirection::North => (0, 1, 0),
            CardinalDirection::West => (-1, 0, 0),
            CardinalDirection::South => (0, -1, 0),
            CardinalDirection::East => (1, 0, 0),
        }
    }
}

pub struct MyWorld {
  
   
 
}

pub struct Terrain{
 
    pub voxeltile_grid: RTree<Voxel>,
    pub voxeltile_diffs: RTree<Voxel>,
}

pub struct ServerStuff{
    pub input_queue: HashMap<EntityID, ActionType>,
    pub output_queue: RTree<ActionPacket>,

    pub sid_eid_map: HashMap<AccountID, EntityID>,
    pub entity_socketid_map: HashMap<EntityID, AccountID>,
    pub server_seed: u64,


}

pub struct Components{
    pub entities: HashSet<EntityID>,
    pub positions: RTree<PositionComponent>,
    pub ent_loc_index: HashMap<EntityID, MyPoint>, //xyz
    pub healths: HashMap<EntityID, HealthComponent>,
    pub entity_types: HashMap<EntityID, EntityType>,
    pub entity_counter: u64,

}

#[derive(Clone, Debug, PartialEq)]
pub struct PositionComponent {
    pub entity_id: EntityID,
    pub point: MyPoint,
}

impl RTreeObject for PositionComponent {
    type Envelope = AABB<(i64, i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.point.0, self.point.1, self.point.2))
    }
}

impl PointDistance for PositionComponent {
    fn distance_2(&self, point: &(i64, i64, i64)) -> i64 {
        self.point.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64, i64)) -> bool {
        self.point.contains_point(point)
    }
}

#[derive(Clone, Debug, )]
pub struct HealthComponent {
    pub cur_health: Health,
    pub max_health: Health,
}

#[derive(Clone, Debug,  PartialEq)]
pub struct Voxel {
    pub roof: RoofType,
    pub floor: FloorType,
    pub furniture: FurnitureType,


    pub voxel_pos: MyPoint,
}

//FIX ALL THIS STUFF
//FIX ALL THIS STUFF
//FIX ALL THIS STUFF
impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character = ",".into();
        let voxel_color = Color::Green;
        let floor_color = Color::White;

        (voxel_character, voxel_color, floor_color)
    }
}

impl RTreeObject for Voxel {
    type Envelope = AABB<(i64, i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.voxel_pos.0, self.voxel_pos.1, self.voxel_pos.2))
    }
}

impl PointDistance for Voxel {
    fn distance_2(&self, point: &(i64, i64, i64)) -> i64 {
        self.voxel_pos.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64, i64)) -> bool {
        self.voxel_pos.contains_point(point)
    }
}

#[derive(Clone, Debug, )]
pub enum EntityType {
    Player,
    Item,
    Monster,
}



#[derive(Clone, Debug,PartialEq)]
pub enum ActionType {
    Wait,
    Take(Accusative),
    Give(Accusative, Dative),
    Hit(Accusative, Instrumental),
    Go(Locative),
    Quit,
}

#[derive(Clone, Debug, )]
pub enum SuccessType {
    Success,
    Failure,
}
#[derive(Clone, Debug, )]
pub struct ActionPacket {
    pub action: ActionType,
    pub success: SuccessType,
    pub action_location: MyPoint,
    pub action_subject: EntityID,
}

impl RTreeObject for ActionPacket {
    type Envelope = AABB<(i64, i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((
            self.action_location.0,
            self.action_location.1,
            self.action_location.2,
        ))
    }
}

impl PointDistance for ActionPacket {
    fn distance_2(&self, point: &(i64, i64, i64)) -> i64 {
        self.action_location.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64, i64)) -> bool {
        self.action_location.contains_point(point)
    }
}

#[derive(Clone, Debug, )]
pub struct Action {}

impl Action {
    pub fn go(world: &mut MyWorld, subject: &EntityID, destination: &Locative) -> SuccessType {
        println!("WAIT WHATTT");

        match destination {
            Locative::Cardinal(cd) => world.move_entity_in_direction(subject, cd),
            Locative::Entity(_) => {
                panic!("not implemented")
            }
            Locative::Point(_) => {
                panic!("not implemented")
            }
        }
    }
}
