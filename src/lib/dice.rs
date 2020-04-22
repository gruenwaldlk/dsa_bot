use crate::lib;
use crate::util;
use log::error;
use rand::prelude::*;
use regex::Regex;
use std::error;
use std::fmt;
use std::str::FromStr;

lazy_static! {
    static ref BASIC_DICE_REGEX: Regex =
        Regex::new(r"^(.*)(d|D|w|W)(\d+)(.*)$").expect("The regex could not be parsed.");
    static ref DICE_NO_MOD_REGEX: Regex =
        Regex::new(r"^(d|D|w|W)(\d+)$").expect("The regex could not be parsed.");
    static ref DICE_WITH_MOD_REGEX: Regex =
        Regex::new(r"^(d|D|w|W)(\d+)(\+|-)(\d+)$").expect("The regex could not be parsed.");
}

#[derive(Debug, Clone)]
pub(crate) struct DiceParseError {
    message: String,
}
impl fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl error::Error for DiceParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
#[derive(Debug)]
pub(crate) struct Dice {
    pub(self) sides: u8,
    pub(self) modifier: lib::modifier::Modifier,
}

impl Dice {
    pub(crate) fn new(sides: u8) -> Self {
        Dice {
            sides,
            modifier: lib::modifier::Modifier::default(),
        }
    }
    #[allow(dead_code)]
    pub(crate) fn with_mod(&self, modifier: lib::modifier::Modifier) -> Self {
        Dice {
            sides: self.sides,
            modifier,
        }
    }
    pub(crate) fn roll(&self) -> u8 {
        thread_rng().gen_range(0, self.sides) + 1
    }
    pub(crate) fn roll_with_mod(&self) -> u8 {
        self.modifier.apply_to_value(self.roll())
    }
    pub(crate) fn roll_n_times(&self, n: u8, vec: &mut Vec<u8>) {
        for _ in 0..n {
            vec.push(self.roll());
        }
    }
    pub(crate) fn roll_n_times_with_mod_per_dice(&self, n: u8, vec: &mut Vec<u8>) {
        for _ in 0..n {
            vec.push(self.roll_with_mod());
        }
    }
}

impl Default for Dice {
    fn default() -> Self {
        Dice {
            sides: 6,
            modifier: lib::modifier::Modifier::default(),
        }
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            util::localisation::get_text("lib.dice.display"),
            self.sides,
            self.modifier
        )
    }
}

impl FromStr for Dice {
    type Err = DiceParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = util::input::remove_whitespace(s);
        if BASIC_DICE_REGEX.is_match(&pattern) {
            let mut dice_pattern = String::new();
            for bc in BASIC_DICE_REGEX.captures_iter(&pattern) {
                dice_pattern = String::new();
                dice_pattern.push_str(&bc[2]);
                dice_pattern.push_str(&bc[3]);
                dice_pattern.push_str(&bc[4]);
            }
            if DICE_NO_MOD_REGEX.is_match(&dice_pattern) {
                let mut sides = 0;
                for dc_no_mod in DICE_NO_MOD_REGEX.captures_iter(&dice_pattern) {
                    sides = u8::from_str(&dc_no_mod[2]).unwrap_or(0);
                }
                return Ok(Dice::new(sides));
            } else if DICE_WITH_MOD_REGEX.is_match(&dice_pattern) {
                let mut sides = 0;
                for dc_no_mod in DICE_WITH_MOD_REGEX.captures_iter(&dice_pattern) {
                    sides = u8::from_str(&dc_no_mod[2]).unwrap_or(0);
                }
                return Ok(Dice::new(sides).with_mod(
                    lib::modifier::Modifier::from_str(&dice_pattern).unwrap_or_default(),
                ));
            }
            error!(
                "Matched the basic dice regex, but no specific dice type could be parsed from {}",
                pattern
            );
            return Err(DiceParseError {
                message: format!(
                    "Matched the basic dice regex, but no specific dice type could be parsed from {}",
                    pattern
                ),
            });
        }
        error!("Could not parse the dice from {}", pattern);
        Err(DiceParseError {
            message: format!("Could not parse the dice from {}", pattern),
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_str_2w20() {
        let actual = Dice::from_str(" 3w20 ").unwrap();
        let expected = Dice::new(20);
        assert_eq!(actual.sides, expected.sides);
    }
    #[test]
    fn test_from_str_w20() {
        let actual = Dice::from_str(" w20 ").unwrap();
        let expected = Dice::new(20);
        assert_eq!(actual.sides, expected.sides);
    }
    #[test]
    fn test_from_str_4w20() {
        let actual = Dice::from_str(" 4w20 ").unwrap();
        let expected = Dice::new(20);
        assert_eq!(actual.sides, expected.sides);
    }

    #[test]
    fn test_from_str_4w20_with_mod_plus4() {
        let actual = Dice::from_str(" 4w20 +4 ").unwrap();
        let expected_dice = Dice::new(20);
        let expected_mod = lib::modifier::Modifier::new(4, lib::operator::Operator::Plus);
        assert_eq!(actual.sides, expected_dice.sides);
        assert_eq!(actual.modifier.value(), expected_mod.value());
        assert_eq!(actual.modifier.operator(), expected_mod.operator());
    }
    #[test]
    fn test_from_str_4w20_with_mod_minus4() {
        let actual = Dice::from_str(" 4w20-4 ").unwrap();
        let expected_dice = Dice::new(20);
        let expected_mod = lib::modifier::Modifier::new(4, lib::operator::Operator::Minus);
        assert_eq!(actual.sides, expected_dice.sides);
        assert_eq!(actual.modifier.value(), expected_mod.value());
        assert_eq!(actual.modifier.operator(), expected_mod.operator());
    }
}
