use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub roof: RoofType,
    pub floor: FloorType,
    pub furniture: FurnitureType,

    pub voxel_pos: MyPoint,
}

//FIX ALL THIS STUFF
//FIX ALL THIS STUFF
//FIX ALL THIS STUFF
impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character = ",".into();
        let voxel_color = Color::Green;
        let floor_color = Color::White;

        (voxel_character, voxel_color, floor_color)
    }
}

impl RTreeObject for Voxel {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.voxel_pos.0, self.voxel_pos.1))
    }
}

impl PointDistance for Voxel {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.voxel_pos.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.voxel_pos.contains_point(point)
    }
}
