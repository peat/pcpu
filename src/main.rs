use crate::circuits::*;

mod circuits;
mod gates;

fn main() {
    let mut rca = RippleCarryAdder::new(4);
    rca.a = vec![false, true, false, true];
    rca.b = vec![false, true, false, false];
    rca.exec();
    println!("RippleCarryAdder");
    println!("  {:?}", rca.a);
    println!("+ {:?}", rca.b);
    println!("= {:?}", rca.output);

    println!();

    let mut rbs = RippleBorrowSubtractor::new(4);
    rbs.a = vec![true, true, false, true];
    rbs.b = vec![true, false, true, false];
    rbs.exec();
    println!("RippleBorrowSubtractor:");
    println!("  {:?}", rbs.a);
    println!("- {:?}", rbs.b);
    println!("= {:?}", rbs.output);
}
