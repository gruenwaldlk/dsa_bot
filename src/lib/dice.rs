use crate::lib::add_without_overflow;
use crate::lib::subtract_without_overflow;
use crate::lib::Operator;

use rand::prelude::*;
use std::fmt::*;

pub(crate) struct Dice {
    pub(self) sides: u8,
    pub(self) mod_value: u8,
    pub(self) mod_op: Operator,
}

impl Dice {
    pub(crate) fn new(sides: u8) -> Self {
        Dice {
            sides,
            mod_value: 0,
            mod_op: Operator::NoP,
        }
    }
    pub(crate) fn with_mod(&self, mod_value: u8, mod_op: Operator) -> Self {
        Dice {
            sides: self.sides,
            mod_value,
            mod_op,
        }
    }
    pub(crate) fn roll(&self) -> u8 {
        let mut rng = thread_rng();
        let val = rng.gen_range(0, self.sides) + 1;
        match self.mod_op {
            Operator::NoP => val,
            Operator::Plus => add_without_overflow(val, self.mod_value),
            Operator::Minus => subtract_without_overflow(val, self.mod_value),
        }
    }
    pub(crate) fn roll_n_times(&self, n: u8, vec: &mut Vec<u8>) {
        for _ in 0..n {
            vec.push(self.roll());
        }
    }
}

impl Default for Dice {
    fn default() -> Self {
        Dice {
            sides: 6,
            mod_value: 0,
            mod_op: Operator::NoP,
        }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "d{}", self.sides)
    }
}
