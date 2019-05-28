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
    pub fn exec(self, a: bool, b: bool) -> bool {
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

    fn not(a: bool) -> bool {
        !a
    }

    fn and(a: bool, b: bool) -> bool {
        a && b
    }

    fn or(a: bool, b: bool) -> bool {
        a || b
    }

    fn nand(a: bool, b: bool) -> bool {
        Logic::not(Logic::and(a, b))
    }

    fn nor(a: bool, b: bool) -> bool {
        Logic::not(Logic::or(a, b))
    }

    fn xor(a: bool, b: bool) -> bool {
        a != b
    }

    fn xnor(a: bool, b: bool) -> bool {
        Logic::not(Logic::xor(a, b))
    }
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub logic: Logic,
    pub a: bool,
    pub b: bool,
    pub output: bool,
}

impl Gate {
    // creates a new gate with the inputs set low
    pub fn new(logic: Logic) -> Gate {
        let a = false;
        let b = false;

        Self {
            logic,
            a,
            b,
            output: logic.exec(a, b),
        }
    }

    pub fn exec(&mut self, a: bool, b: bool) {
        self.a = a;
        self.b = b;
        self.output = self.logic.exec(a, b);
    }
}
