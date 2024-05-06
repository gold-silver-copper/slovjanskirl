use crate::*;


#[derive(Clone, Debug)]
pub struct MyWorld {
    pub terrain: Terrain,
    pub server_stuff: ServerStuff,
    pub components: Components,
    pub world_seed: i64,
}

impl Default for MyWorld {
    fn default() -> Self {
        let rngik = 100;

        Self {
            terrain: Terrain::new(rngik.clone()),
            server_stuff: ServerStuff::default(),
            components: Components::default(),
            world_seed: rngik.clone(),
        }
    }
}
