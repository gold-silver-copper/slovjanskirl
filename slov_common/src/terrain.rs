use crate::*;

#[derive(Clone, Debug)]
pub struct Terrain {
    pub voxeltile_grid: RTree<Voxel>,
    
}

impl Terrain {
    pub fn new(rngik: u32) -> Terrain {
        Terrain {
            voxeltile_grid: MyWorld::generate_test(rngik),
      
        }
    }
}
