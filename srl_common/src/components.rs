use crate::*;

#[derive(Clone, Debug)]
pub struct Components {
    pub entities: HashSet<EntityID>,
    pub positions: RTree<PositionComponent>,
    pub ent_loc_index: HashMap<EntityID, MyPoint>, //xyz
    pub healths: HashMap<EntityID, HealthComponent>,
    pub entity_types: HashMap<EntityID, EntityType>,
    pub entity_counter: u64,
}

impl Default for Components {
    fn default() -> Self {
        Self {
            entities: HashSet::new(),
            positions: RTree::new(),
            ent_loc_index: HashMap::new(),
            healths: HashMap::new(),
            entity_types: HashMap::new(),
            entity_counter: 1,
        }
    }
}

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

#[derive(Clone, Debug)]
pub struct HealthComponent {
    pub cur_health: Health,
    pub max_health: Health,
}
