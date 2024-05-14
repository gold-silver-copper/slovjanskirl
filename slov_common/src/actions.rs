use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Wait,
    Take(AccusativeID),
    Drop(Item),
    Give(AccusativeID, DativeID),
    Hit(AccusativeID, InstrumentalID),
    Go(LocativeID),
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SuccessType {
    Success,
    Failure,
}
#[derive(Clone, Debug)]
pub struct ActionPacket {
    pub action: ActionType,
    pub success: SuccessType,
    pub action_location: MyPoint,
    pub action_subject: EntityID,
}

impl RTreeObject for ActionPacket {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.action_location.0, self.action_location.1))
    }
}

impl PointDistance for ActionPacket {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.action_location.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.action_location.contains_point(point)
    }
}

#[derive(Clone, Debug)]
pub struct Action {}

impl Action {
    pub fn go(world: &mut MyWorld, subject: &EntityID, destination: &LocativeID) -> SuccessType {
        println!("WAIT WHATTT");

        match destination {
            LocativeID::Cardinal(cd) => world.move_entity_in_direction(subject, cd), //world.move_entity_in_direction(subject, cd),
            LocativeID::Entity(_) => {
                panic!("not implemented ent")
            }
            LocativeID::Point(_) => {
                panic!("not implemented point")
            }
        }
    }
   /*
    pub fn attack(world: &mut MyWorld, subject: &EntityID, destination: &LocativeID) -> SuccessType {
        println!("WAIT WHATTT");

        match destination {
            LocativeID::Cardinal(cd) => world.attack_entity_in_direction(subject, cd), //world.move_entity_in_direction(subject, cd),
            LocativeID::Entity(_) => {
                panic!("not implemented ent")
            }
            LocativeID::Point(_) => {
                panic!("not implemented point")
            }
        }
    }
   
    */
    pub fn take(world: &mut MyWorld, subject: &EntityID, object: &EntityID) -> SuccessType {
        //get subj and obj locations
        let sub_loc = world.ent_loc_index.get(subject);
        let obj_loc = world.ent_loc_index.get(object);

        //if the subj and obj are in the same place
        if sub_loc == obj_loc {
            let mut nun = EntityType::None;
            let mut nun2 = EntityType::None;

            let mut itik = world.entity_map.get(object).unwrap_or(&mut nun2).clone();
            let mut boop = world.entity_map.get_mut(subject).unwrap_or(&mut nun);
            //if the object is an item
            match itik {
                EntityType::Item(itimik) => {
                    match boop {
                        //if the subject is a player
                        EntityType::Human(pla) => {
                            pla.inventory.push(itimik.clone());
                            world.delete_entity(object);
                            return SuccessType::Success;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        SuccessType::Failure
    }
    pub fn drop(world: &mut MyWorld, subject: &EntityID, item_to_drop: &Item) -> SuccessType {
        let sub_loc = world.ent_loc_index.get(subject).unwrap_or(&(0, 0)).clone();

        //does ent contain item

        let mut nun = EntityType::None;

        let mut meow = SuccessType::Failure;

        let ent_to_check = world.entity_map.get_mut(subject).unwrap_or(&mut nun);

        match ent_to_check {
            EntityType::Human(igrok) => {
                meow = remove_first_instance(&mut igrok.inventory, &item_to_drop.clone());
            }
            _ => (),
        }

        if meow == SuccessType::Success {
            world.new_entity(&sub_loc, &EntityType::Item(item_to_drop.clone()));
        }

        meow
    }
}
