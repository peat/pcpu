use crate::gates::*;

#[derive(Debug)]
pub struct HalfAdder {
    pub a: Voltage,
    pub b: Voltage,
    pub sum: Voltage,
    pub carry: Voltage,
    xor_gate: Gate,
    and_gate: Gate,
}

impl HalfAdder {
    // initialize with inputs low
    pub fn new() -> Self {
        Self {
            a: Voltage::Low,
            b: Voltage::Low,
            sum: Voltage::Low,
            carry: Voltage::Low,
            xor_gate: Gate::new(Logic::XOR),
            and_gate: Gate::new(Logic::AND),
        }
    }

    pub fn exec(&mut self) {
        self.xor_gate.exec(self.a, self.b);
        self.and_gate.exec(self.a, self.b);
        self.sum = self.xor_gate.output;
        self.carry = self.and_gate.output;
    }
}

