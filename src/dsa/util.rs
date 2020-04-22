use crate::lib;
use crate::util;
use log::debug;

pub(crate) fn check_critical_success(attribute: u8, crit_count: u8) -> u8 {
    let d = lib::dice::Dice::new(20);
    let roll = d.roll();
    debug!(
        "fn check_critical_success({}, {}) -> rolled {}.",
        attribute, crit_count, roll
    );
    if roll == 1 {
        check_critical_success(attribute, util::uint8::add_without_overflow(crit_count, 1))
    } else if roll <= attribute {
        util::uint8::add_without_overflow(crit_count, 1)
    } else {
        crit_count
    }
}

pub(crate) fn check_critical_failure(attribute: u8, crit_count: u8) -> u8 {
    let d = lib::dice::Dice::new(20);
    let roll = d.roll();
    debug!(
        "fn check_critical_success({}, {}) -> rolled {}.",
        attribute, crit_count, roll
    );
    if roll == 20 {
        check_critical_success(attribute, util::uint8::add_without_overflow(crit_count, 1))
    } else if roll <= attribute {
        crit_count
    } else {
        util::uint8::add_without_overflow(crit_count, 1)
    }
}
