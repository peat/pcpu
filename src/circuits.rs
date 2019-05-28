use crate::gates::*;

#[derive(Debug, Clone)]
pub struct HalfAdder {
    pub a: bool,
    pub b: bool,
    pub sum: bool,
    pub carry: bool,
    xor_gate: Gate,
    and_gate: Gate,
}

impl HalfAdder {
    // initialize with inputs low
    pub fn new() -> Self {
        Self {
            a: false,
            b: false,
            sum: false,
            carry: false,
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

#[derive(Debug, Clone)]
pub struct FullAdder {
    pub a: bool,
    pub b: bool,
    pub sum: bool,
    pub carry_in: bool,
    pub carry_out: bool,
    half_adder_a: HalfAdder,
    half_adder_b: HalfAdder,
    or_gate: Gate,
}

impl FullAdder {
    // initialize with inputs low
    pub fn new() -> Self {
        Self {
            a: false,
            b: false,
            sum: false,
            carry_in: false,
            carry_out: false,
            half_adder_a: HalfAdder::new(),
            half_adder_b: HalfAdder::new(),
            or_gate: Gate::new(Logic::OR),
        }
    }

    pub fn exec(&mut self) {
        // add the primary bits
        self.half_adder_a.a = self.a;
        self.half_adder_a.b = self.b;
        self.half_adder_a.exec();

        // add the sum from above to the carried in bit
        self.half_adder_b.a = self.half_adder_a.sum;
        self.half_adder_b.b = self.carry_in;
        self.half_adder_b.exec();

        // the sum bit is the result of the above adder
        self.sum = self.half_adder_b.sum;

        // the carry out bit is an XOR of the first or second carry bits
        self.or_gate
            .exec(self.half_adder_a.carry, self.half_adder_b.carry);
        self.carry_out = self.or_gate.output;

    }
}

#[derive(Debug)]
pub struct RippleCarryAdder {
    pub a: Vec<bool>,
    pub b: Vec<bool>,
    pub full_adders: Vec<FullAdder>,
    pub output: Vec<bool>,
}

impl RippleCarryAdder {
    pub fn new(size: usize) -> Self {
        Self {
            a: vec![false; size],
            b: vec![false; size],
            full_adders: vec![FullAdder::new(); size],
            output: vec![false; size],
        }
    }

    pub fn exec(&mut self) {
        self.output = vec![];
        let mut pairs = self.a.iter().zip(self.b.iter());
        let mut carry = false;

        for adder in self.full_adders.iter_mut() {
            if let Some((a_bit, b_bit)) = pairs.next() {
                adder.carry_in = carry;
                adder.a = *a_bit;
                adder.b = *b_bit;
                adder.exec();
                self.output.push(adder.sum);
                carry = adder.carry_out;
            }
        }
    }
}
