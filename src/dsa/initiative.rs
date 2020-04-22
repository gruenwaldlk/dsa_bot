use crate::lib;
use crate::util;
use std::fmt::*;

pub(crate) struct Ini {
    base: u8,
    roll: u8,
    modifier: lib::modifier::Modifier,
}

impl Ini {
    pub(crate) fn new(base: u8, roll: u8, modifier: lib::modifier::Modifier) -> Self {
        Self {
            base,
            roll,
            modifier,
        }
    }
}

impl Display for Ini {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.modifier
                .apply_to_value(util::uint8::add_without_overflow(self.base, self.roll))
        )
    }
}
