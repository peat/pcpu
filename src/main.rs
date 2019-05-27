use crate::circuits::*;
use crate::gates::{Gate, Logic, Voltage};

mod circuits;
mod gates;

fn main() {
    let mut ha = HalfAdder::new();

    ha.a = Voltage::Low;
    ha.b = Voltage::Low;
    ha.exec();
    println!("{:?}", ha);

    ha.a = Voltage::High;
    ha.b = Voltage::Low;
    ha.exec();
    println!("{:?}", ha);

    ha.a = Voltage::Low;
    ha.b = Voltage::High;
    ha.exec();
    println!("{:?}", ha);

    ha.a = Voltage::High;
    ha.b = Voltage::High;
    ha.exec();
    println!("{:?}", ha);
}
