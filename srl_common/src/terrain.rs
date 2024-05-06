use crate::*;

#[derive(Clone, Debug)]
pub struct Terrain {
    pub voxeltile_grid: RTree<Voxel>,
    pub voxeltile_diffs: RTree<Voxel>,
}

impl Terrain {
    pub fn new(rngik: i64) -> Terrain {
        Terrain {
            voxeltile_grid: MyWorld::init_world(),
            voxeltile_diffs: RTree::default(),
        }
    }
}
