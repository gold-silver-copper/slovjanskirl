use crate::*;
#[derive(Clone, Debug, Display, PartialEq)]
pub enum MeleeWeaponType {
    Nož,
    Sěkyra,
    Kyj,
    Meč,
    Kopje,
    Cepj,

     Nagajka,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum RangedWeaponType {
    Lųk,
   Proca,
   Prak,
    Prašča,
      Šlojder,
      Kuša,
    Samostrěl,
    Kameni,
   Arbalet,
}


#[derive(Clone, Debug, Display, PartialEq)]
pub enum StoneType {
    Granit,
    Kremenj,
    Rubin,
    Mramor,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum MetalType {
    Bronza,
    Zlåto,
    Železo,
    Srebro,
    Medj,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum WoodType {
    Brest,
    Jasenj,
    Lipa,
    Jablanj,
    Kalina,
    Jalovec,
    Brek,
    Kaštan,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum PlantType {
    Trava,
    Kovylj, //needle grass
    Burjan, // high grass
    Kanabis,
    Jasenėc,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum BushType {
    Klubnika,
    Jagoda,     //needle grass
    Zemljanika, // high grass
    Ježina,
    Kųpina,
    Brusnica,
    Malina,
    Kljukva,
    Črnica,
    Žuravina,
    Bȯzina,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum MammalType {
    Los,
    Jelenj,
    Krava,
    Pes,
    Tigr,
}


#[derive(Clone, Debug, Display, PartialEq)]
pub enum FishType {
    Losos,
    Tunec,
    Karas,
}


#[derive(Clone, Debug, Display, PartialEq)]
pub enum BirdType {
    Sova,
    Vrabec,
    Vran,
    Gavran,
    Kos,
    Gųsę,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LizardType {
    Gad,
    Jaščer,
    Iguana,
    Vųž,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum AnimalPartType {
  
    Meat,
    Feather,
    Skin,
    Hair,
    Bone,
}



#[derive(Clone, Debug, Display, PartialEq)]
pub enum ClothingItemType {
    Pantalony, // Pants
    Hlače,     // Trousers
    Suknja,    // Skirt

    Šarovary,  // Baggy pants
    Spodnjice, // Underpants

    Košula, // Shirt
    Tunika, // Tunic
    Halja,  // Dress
    Bluza,  // Blouse
    Majica, // T-shirt
    Kofta,  // Sweater/Cardigan

    Kabanica, // Raincoat
    Kožuh,    // Fur coat
    Vesta,    // Vest
    Šal,

    Šarf,
    Kosynka, //headscarf
    Platok, //headscarf
    

    Šátek,
    Pléť, //big piece of cloth
    Ruta, //headscarf
    Ogrinjalo, //headscarf
    Marama, //headscarf
    Voalj, //veil
    Závoj,  //veil
    Karpa,
    Palantin,

    Šapka,
    Šljem,
    Kapela,
    Kapuc, //kapushon hood
    Beretka,
    Koĺčuga,
    Bandana,
    Vual,
    Klobuk,
    Šešir,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum MebeljType {
    Stěna,
    Stol,
    Stul,
    Skrinja, //sunduk
    Dvėrj,
    Vråta,
    Vaza,
    Škaf,
}