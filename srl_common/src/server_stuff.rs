use crate::*;


#[derive(Clone, Debug)]
pub struct ServerStuff {
    pub input_queue: HashMap<EntityID, ActionType>,
    pub output_queue: RTree<ActionPacket>,

    pub entity_accid_map: HashMap<EntityID, AccountID>,
}

impl Default for ServerStuff {
    fn default() -> Self {
        Self {
            input_queue: HashMap::default(),
            output_queue: RTree::default(),
          
            entity_accid_map: HashMap::default(),
        }
    }
}
