use slov_common::*;
fn main() {
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
    //  println!("typik is {}", typik);
}
