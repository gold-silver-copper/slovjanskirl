use slov_common::*;
fn main() {
    let boop: MyPoint = (5, 5);
    let nw = MyWorld::default();
    let swordik = MeleeWeaponType::Sěkyra;

    println!("{:#?}", boop);
    println!("{:#?}", nw);
    println!("swordik is {}", swordik);
  //  println!("typik is {}", typik);
}
