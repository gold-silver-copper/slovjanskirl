use srl_common::*;
fn main() {
    let boop: MyPoint = (5, 5);
    let nw = MyWorld::default();

    println!("{:#?}", boop);
    println!("{:#?}", nw);
}
