use crate::gates::*;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FullAdder {
    pub a: Voltage,
    pub b: Voltage,
    pub sum: Voltage,
    pub carry_in: Voltage,
    pub carry_out: Voltage,
    half_adder_a: HalfAdder,
    half_adder_b: HalfAdder,
    or_gate: Gate,
}

impl FullAdder {
    // initialize with inputs low
    pub fn new() -> Self {
        Self {
            a: Voltage::Low,
            b: Voltage::Low,
            sum: Voltage::Low,
            carry_in: Voltage::Low,
            carry_out: Voltage::Low,
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
    pub a: Vec<Voltage>,
    pub b: Vec<Voltage>,
    pub full_adders: Vec<FullAdder>,
    pub output: Vec<Voltage>,
}

impl RippleCarryAdder {
    pub fn new(size: usize) -> Self {
        Self {
            a: vec![Voltage::Low; size],
            b: vec![Voltage::Low; size],
            full_adders: vec![FullAdder::new(); size],
            output: vec![Voltage::Low; size],
        }
    }

    pub fn exec(&mut self) {
        self.output = vec![];
        let mut pairs = self.a.iter().zip(self.b.iter());
        let mut carry = Voltage::Low;

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
