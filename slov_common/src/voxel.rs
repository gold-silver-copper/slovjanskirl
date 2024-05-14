use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub roof: Roof,
    pub floor: Floor,

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
    Water,
    LightGrass,
    DarkGrass,
    Sand,
}

impl Floor {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Air => Color::Rgb(239, 240, 235),
            Self::Dirt => Color::Rgb(155, 118, 83),
            Self::Water => Color::Rgb(15, 94, 156),
            Self::LightGrass => Color::Rgb(65, 152, 1),
            Self::DarkGrass => Color::Rgb(19, 109, 21),
            Self::Sand => Color::Rgb(242, 210, 169),
        }
    }

    pub fn to_displaychar(&self) -> String {
        match &self {
            Self::Air => " ".into(),
            Self::Dirt => " ".into(),
            Self::Water => "~".into(),
            Self::LightGrass => ",".into(),
            Self::DarkGrass => "\"".into(),
            Self::Sand => ".".into(),
        }
    }
   
    pub fn to_front_color(&self) -> Color {
        match &self {
            Self::Air => Color::Rgb(200, 240, 235),
            Self::Dirt => Color::Rgb(130, 118, 83),
            Self::Water => Color::Rgb(40, 94, 156),
            Self::LightGrass => Color::Rgb(40, 152, 1),
            Self::DarkGrass => Color::Rgb(20, 130, 21),
            Self::Sand => Color::Rgb(222, 210, 169),
        }
    }
}

impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character: String = self.floor.to_displaychar();
        let char_color = self.floor.to_front_color();

 
        let floor_color = self.floor.to_color();

        (voxel_character, char_color, floor_color)
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
