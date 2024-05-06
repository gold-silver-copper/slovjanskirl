use crate::*;


pub struct Terrain{
 
    pub voxeltile_grid: RTree<Voxel>,
    pub voxeltile_diffs: RTree<Voxel>,
}
