use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub roof: Roof,
    pub floor: Floor,
    pub furniture: Furniture,

    pub voxel_pos: MyPoint,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Roof {
    Air,
}
impl Roof {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Air => Color::Rgb(239, 240, 235),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Floor {
    Air,
    Dirt,
}

impl Floor {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Air => Color::Rgb(239, 240, 235),
            Self::Dirt => Color::Rgb(155,118,83)

        }
    }
}



//FIX ALL THIS STUFF
//FIX ALL THIS STUFF
//FIX ALL THIS STUFF
impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character = self.furniture.to_char();
        let voxel_color = self.furniture.to_color();
        let floor_color = self.floor.to_color();

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
