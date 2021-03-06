pub mod attribute;
pub mod char_repository;
pub mod character;
pub mod talent;
use crate::lib::dice::Dice;
use crate::lib::*;
use log::debug;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum ResultType {
  Failure,
  CriticalFailure,
  Success,
  CriticalSuccess,
}

fn check_critical_success(attribute: u8, crit_count: u8) -> u8 {
  let d = Dice::new(20);
  let roll = d.roll();
  debug!(
    "fn check_critical_success({}, {}) -> rolled {}.",
    attribute, crit_count, roll
  );
  if roll == 1 {
    check_critical_success(attribute, add_without_overflow(crit_count, 1))
  } else if roll <= attribute {
    add_without_overflow(crit_count, 1)
  } else {
    crit_count
  }
}

fn check_critical_failure(attribute: u8, crit_count: u8) -> u8 {
  let d = Dice::new(20);
  let roll = d.roll();
  debug!(
    "fn check_critical_success({}, {}) -> rolled {}.",
    attribute, crit_count, roll
  );
  if roll == 20 {
    check_critical_success(attribute, add_without_overflow(crit_count, 1))
  } else if roll <= attribute {
    crit_count
  } else {
    add_without_overflow(crit_count, 1)
  }
}
