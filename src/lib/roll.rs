use crate::lib;
use crate::util;
use regex::Regex;
use std::error;
use std::fmt;
use std::str::FromStr;

lazy_static! {
    static ref BASIC_ROLL_REGEX: Regex =
        Regex::new(r"^(.*)(\d*)(d|D|w|W)(\d+)(.*)$").expect("The regex could not be parsed.");
    static ref ROLL_NO_MOD_REGEX: Regex =
        Regex::new(r"^(\d*)(d|D|w|W)(\d+)$").expect("The regex could not be parsed.");
    static ref ROLL_WITH_MOD_REGEX: Regex =
        Regex::new(r"^(\d*)(d|D|w|W)(\d+)(\+|-)(\d+)$").expect("The regex could not be parsed.");
}

#[derive(Debug, Clone)]
pub(crate) struct RollParseError {
    message: String,
}
impl fmt::Display for RollParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl error::Error for RollParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub(crate) struct Roll {
    pub(self) number_of_rolls: u8,
    pub(self) dice: lib::dice::Dice,
}

impl Roll {
    pub(crate) fn new(number_of_rolls: u8, dice: lib::dice::Dice) -> Self {
        Self {
            number_of_rolls,
            dice,
        }
    }
    pub(crate) fn number_of_rolls(&self) -> u8 {
        self.number_of_rolls
    }
    pub(crate) fn dice(&self) -> &lib::dice::Dice {
        &self.dice
    }
    pub(crate) fn roll(&self) -> u8 {
        self.dice.roll()
    }
    pub(crate) fn roll_n_times(&self, vec: &mut Vec<u8>) {
        self.dice.roll_n_times(self.number_of_rolls, vec);
    }
}

impl Default for Roll {
    fn default() -> Self {
        Self {
            number_of_rolls: 1,
            dice: lib::dice::Dice::default(),
        }
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.number_of_rolls() {
            0 => String::new(),
            _ => format!("{}", self.number_of_rolls()),
        };
        write!(
            f,
            "{}{}{}",
            s,
            util::localisation::get_text("lib.dice.display"),
            self.dice
        )
    }
}
