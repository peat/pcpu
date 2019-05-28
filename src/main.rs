use crate::circuits::*;
use crate::gates::*;

mod circuits;
mod gates;

fn main() {
    let mut rca = RippleCarryAdder::new(4);

    println!("{:?}\n", rca);

    rca.a = vec![Voltage::Low, Voltage::High, Voltage::Low, Voltage::High];
    rca.b = vec![Voltage::Low, Voltage::High, Voltage::Low, Voltage::Low];
    rca.exec();
    println!("{:?}\n", rca);

}
