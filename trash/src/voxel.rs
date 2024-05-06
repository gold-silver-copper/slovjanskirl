use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub floor_type: FloorType,

    pub voxel_pos: MyPoint,
}

impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character = self.voxel_type.grapheme();
        let voxel_color = self.voxel_type.color();
        let floor_color = self.floor_type.color();

        (voxel_character, voxel_color, floor_color)
    }
}

impl RTreeObject for Voxel {
    type Envelope = AABB<(i64, i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.voxel_pos.0, self.voxel_pos.1, self.voxel_pos.2))
    }
}

impl PointDistance for Voxel {
    fn distance_2(&self, point: &(i64, i64, i64)) -> i64 {
        self.voxel_pos.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64, i64)) -> bool {
        self.voxel_pos.contains_point(point)
    }
}
