use crate::*;
pub struct ServerStuff{
    pub input_queue: HashMap<EntityID, ActionType>,
    pub output_queue: RTree<ActionPacket>,

    pub sid_eid_map: HashMap<AccountID, EntityID>,
    pub entity_socketid_map: HashMap<EntityID, AccountID>,
    


}

