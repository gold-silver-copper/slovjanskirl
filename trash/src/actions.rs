use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    Wait,
    Take(Accusative),
    Give(Accusative, Dative),
    Hit(Accusative, Instrumental),
    Go(Locative),
    Quit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SuccessType {
    Success,
    Failure,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
