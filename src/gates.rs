#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Voltage {
    High,
    Low,
}

impl Voltage {
    fn bool(self) -> bool {
        self.high()
    }

    fn low(self) -> bool {
        self == Voltage::Low
    }

    fn high(self) -> bool {
        self == Voltage::High
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Logic {
    NOT,
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    XNOR,
}

impl Logic {
    pub fn exec(self, a: Voltage, b: Voltage) -> Voltage {
        match self {
            Logic::NOT => Logic::not(a),
            Logic::AND => Logic::and(a, b),
            Logic::OR => Logic::or(a, b),
            Logic::NAND => Logic::nand(a, b),
            Logic::NOR => Logic::nor(a, b),
            Logic::XOR => Logic::xor(a, b),
            Logic::XNOR => Logic::xnor(a, b),
        }
    }

    fn not(a: Voltage) -> Voltage {
        match a {
            Voltage::High => Voltage::Low,
            Voltage::Low => Voltage::High,
        }
    }

    fn and(a: Voltage, b: Voltage) -> Voltage {
        if a.high() && b.high() {
            return Voltage::High;
        }

        Voltage::Low
    }

    fn or(a: Voltage, b: Voltage) -> Voltage {
        if a.low() && b.low() {
            return Voltage::Low;
        }

        Voltage::High
    }

    fn nand(a: Voltage, b: Voltage) -> Voltage {
        Logic::not(Logic::and(a, b))
    }

    fn nor(a: Voltage, b: Voltage) -> Voltage {
        Logic::not(Logic::or(a, b))
    }

    fn xor(a: Voltage, b: Voltage) -> Voltage {
        if a != b {
            return Voltage::High;
        }

        Voltage::Low
    }

    fn xnor(a: Voltage, b: Voltage) -> Voltage {
        Logic::not(Logic::xor(a, b))
    }
}

#[derive(Debug)]
pub struct Gate {
    pub logic: Logic,
    pub a: Voltage,
    pub b: Voltage,
    pub output: Voltage,
}

impl Gate {
    // creates a new gate with the inputs set low
    pub fn new(logic: Logic) -> Gate {
        let a = Voltage::Low;
        let b = Voltage::Low;

        Self {
            logic,
            a,
            b,
            output: logic.exec(a, b),
        }
    }

    pub fn exec(&mut self, a: Voltage, b: Voltage) {
        self.a = a;
        self.b = b;
        self.output = self.logic.exec(a, b);
    }
}
