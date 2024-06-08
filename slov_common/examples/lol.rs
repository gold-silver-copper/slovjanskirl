use noise::{NoiseFn, Perlin, Seedable};
use slov_common::*;
use serde_derive::Deserialize;
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct Animal {
    id: String,
    isv: String,
    variations: String,
    animal_type: String,
    symbol: String,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
}



fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let parts: Vec<u8> = s.split(',')
        .map(|part| part.trim().parse().unwrap_or(0))
        .collect();

    if parts.len() != 3 {
        return Err(serde::de::Error::custom("invalid color format"));
    }

    Ok(Color::Rgb(parts[0],parts[1],parts[2]))
}


fn main() {
    let kost = "kost";
    let dom = "dom";
    let kostik = "kost́";

    println!(" kost is {:#?}", ISV::guess_gender(kost));
    println!(" dom is {:#?}", ISV::guess_gender(dom));
    println!(" kost́ is {:#?}", ISV::guess_gender(kostik));
    println!("{}", "Žuravina".to_lowercase());
    println!("los is animate? - {:#?}", ISV::noun_is_animate("los"));
    println!(
        "jablanj is animate? - {:#?}",
        ISV::noun_is_animate("jablanj")
    );
    println!("{}", ISV::acc_sg("dom"));
    println!("{}", ISV::acc_sg("deva"));
    println!("{}", ISV::acc_sg("masina"));

    println!("{}", ISV::acc_sg("jelenj"));

    println!("{}", ISV::dat_sg("dom"));
    println!("{}", ISV::dat_sg("deva"));
    println!("{}", ISV::dat_sg("pivo"));
    println!("{}", ISV::dat_sg("masina"));

    println!("{}", ISV::dat_sg("jelenj"));

    println!("{}", ISV::ins_sg("dom"));
    println!("{}", ISV::ins_sg("deva"));
    println!("{}", ISV::ins_sg("pivo"));
    println!("{}", ISV::ins_sg("masina"));

    println!("{}", ISV::ins_sg("jelenj"));

    let csv = File::open("../assets/data/isv_animals.csv").unwrap();

    let mut reader = csv::Reader::from_reader(csv);

    for animal in reader.deserialize() {
        let animal:Animal = animal.unwrap();
       
        println!("{:?}", animal);
    }

    
   

    /*

      let boop: MyPoint = (5, 5);
    let nw = MyWorld::default();
    let swordik = MeleeWeaponType::Sěkyra;
    let myitem = Item {
        item_type: ItemType::Melee(MeleeWeapon {
            weapon_type: MeleeWeaponType::Kopje,
            material_type: Material::Metal(MetalType::Bronza),
        }),
    };

    //  let ne = EntityType::Item(myitem);

    println!("{:#?}", boop);
    println!("{:#?}", nw);
    println!("swordik is {}", swordik);
    println!("myitem is is {}", myitem.to_char());

    let perlin = Perlin::new(1);
    let val = perlin.get([41111.0, 379999.7]);
    println!("val is {}", val);
    //  println!("typik is {}", typik);

     */
}
