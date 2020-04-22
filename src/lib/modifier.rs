use crate::lib;
use crate::util;
use regex::Regex;
use std::error;
use std::fmt;
use std::str::FromStr;

lazy_static! {
    static ref MOD_PATTERN_REGEX: Regex =
        Regex::new(r"^(.*)(\+|-)(\d+)(.*)$").expect("The regex could not be parsed.");
    static ref MOD_PLUS_REGEX: Regex =
        Regex::new(r"^(\+)(\d+)$").expect("The regex could not be parsed.");
    static ref MOD_MINUS_REGEX: Regex =
        Regex::new(r"^(-)(\d+)$").expect("The regex could not be parsed.");
}

#[derive(Debug, Clone)]
pub(crate) struct ModifierError {
    message: String,
}
impl fmt::Display for ModifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl error::Error for ModifierError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
#[derive(Debug)]
pub(crate) struct Modifier {
    value: u8,
    operator: lib::operator::Operator,
}

impl Modifier {
    pub(crate) fn new(value: u8, operator: lib::operator::Operator) -> Self {
        Self { value, operator }
    }
    pub(crate) fn apply_to_value(&self, v: u8) -> u8 {
        match self.operator {
            lib::operator::Operator::Plus => util::uint8::add_without_overflow(v, self.value),
            lib::operator::Operator::Minus => util::uint8::subtract_without_overflow(v, self.value),
            lib::operator::Operator::NoP => v,
        }
    }
}

impl FromStr for Modifier {
    type Err = ModifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = util::input::remove_whitespace(s);
        if MOD_PATTERN_REGEX.is_match(&pattern) {
            let mut modifier = String::new();
            for cap in MOD_PATTERN_REGEX.captures_iter(&pattern) {
                modifier = String::from(&cap[2]);
                modifier.push_str(&cap[3]);
            }
            modifier = util::input::remove_whitespace(&modifier);
            if MOD_PLUS_REGEX.is_match(&modifier) {
                let mut v_plus: u8 = 0;
                for c in MOD_PATTERN_REGEX.captures_iter(&modifier) {
                    v_plus = u8::from_str(&c[3]).unwrap_or(0);
                }
                if v_plus != 0 {
                    return Ok(lib::modifier::Modifier::new(
                        v_plus,
                        lib::operator::Operator::Plus,
                    ));
                }
            } else if MOD_MINUS_REGEX.is_match(&modifier) {
                let mut v_minus = 0;
                for c in MOD_PATTERN_REGEX.captures_iter(&modifier) {
                    v_minus = u8::from_str(&c[3]).unwrap_or(0);
                }
                if v_minus != 0 {
                    return Ok(lib::modifier::Modifier::new(
                        v_minus,
                        lib::operator::Operator::Minus,
                    ));
                }
            }
        }
        Ok(Modifier::default())
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Self {
            value: 0,
            operator: lib::operator::Operator::NoP,
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self.operator {
            lib::operator::Operator::Plus => "+",
            lib::operator::Operator::Minus => "-",
            lib::operator::Operator::NoP => "",
        };
        if self.value == 0 {
            write!(f, "")
        } else {
            write!(f, "({}{})", op, self.value)
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test_from_str_plus9() {
        let mod_actual = Modifier::from_str("3w4+9").unwrap();
        let mod_expected = Modifier::new(9, lib::operator::Operator::Plus);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_minus9() {
        let mod_actual = Modifier::from_str("3w4-9").unwrap();
        let mod_expected = Modifier::new(9, lib::operator::Operator::Minus);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_nop() {
        let mod_actual = Modifier::from_str("3w4+0").unwrap();
        let mod_expected = Modifier::new(0, lib::operator::Operator::NoP);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_no_mod() {
        let mod_actual = Modifier::from_str("3w4").unwrap();
        let mod_expected = Modifier::new(0, lib::operator::Operator::NoP);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_plus9_with_whitespace() {
        let mod_actual = Modifier::from_str("3w4 +9").unwrap();
        let mod_expected = Modifier::new(9, lib::operator::Operator::Plus);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_minus9_with_whitespace() {
        let mod_actual = Modifier::from_str("3w4 -9\n").unwrap();
        let mod_expected = Modifier::new(9, lib::operator::Operator::Minus);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_nop_with_whitespace() {
        let mod_actual = Modifier::from_str(" 3w4\t+0  ").unwrap();
        let mod_expected = Modifier::new(0, lib::operator::Operator::NoP);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
    #[test]
    fn test_from_str_no_mod_with_whitespace() {
        let mod_actual = Modifier::from_str(" 3w4\t").unwrap();
        let mod_expected = Modifier::new(0, lib::operator::Operator::NoP);
        assert_eq!(mod_expected.value, mod_actual.value);
        assert_eq!(mod_expected.operator, mod_actual.operator);
    }
}
