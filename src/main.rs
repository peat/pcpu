use crate::circuits::*;

mod circuits;
mod gates;

fn main() {
    let mut rca = RippleCarryAdder::new(4);
    println!("{:?}\n", rca);

    rca.a = vec![false, true, false, true];
    rca.b = vec![false, true, false, false];
    rca.exec();
    println!("{:?}\n", rca);

}
