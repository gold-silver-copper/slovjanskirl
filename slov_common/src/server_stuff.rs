use crate::*;

#[derive(Clone, Debug)]
pub struct ServerStuff {
    pub input_queue: HashMap<EntityID, ActionType>,
    pub output_queue: RTree<ActionPacket>,
    pub account_counter: u64,

    pub entity_accid_map: HashMap<EntityID, AccountID>,
}

impl Default for ServerStuff {
    fn default() -> Self {
        Self {
            input_queue: HashMap::default(),
            output_queue: RTree::default(),
            account_counter: 1,

            entity_accid_map: HashMap::default(),
        }
    }
}
