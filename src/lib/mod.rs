#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Operator {
    Plus,
    Minus,
    NoP,
}

pub(crate) fn substract_without_overflow(minuend: u8, subtrahend: u8) -> u8 {
    match u8::checked_sub(minuend, subtrahend) {
        Some(v) => v,
        None => u8::min_value(),
    }
}

pub(crate) fn add_without_overflow(s1: u8, s2: u8) -> u8 {
    match u8::checked_add(s1, s2) {
        Some(v) => v,
        None => u8::max_value(),
    }
}

pub mod dice;
