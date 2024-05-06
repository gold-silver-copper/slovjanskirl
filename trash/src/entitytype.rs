use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Item,
    Monster,
}

impl EntityType {
    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = match self {
            EntityType::Item => "i",
            EntityType::Monster => "M",
            EntityType::Player => "𓁇",
        };
        (ent_char.into(), Color::Red, Color::Black)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    // Weapons
    Sabre, // A type of curved sword used widely in Eastern Europe
    Axe,
    ShortBow, // Typical lightweight bow
    Javelin,  // Throwing spear, common among light infantry and skirmishers
}
