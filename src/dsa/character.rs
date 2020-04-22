use crate::dsa;
use crate::dsa::attribute;
use crate::dsa::checks;
use crate::dsa::checks::CheckResult;
use crate::dsa::initiative;
use crate::dsa::result;
use crate::dsa::talent;
use crate::lib;
use crate::util;
use log::info;
use log::warn;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Character {
    pub(self) name: String,
    pub(self) owner: u64,
    pub(self) attributes: HashMap<String, attribute::Attribute>,
    pub(self) talents: HashMap<String, talent::Talent>,
    pub(self) spells: HashMap<String, talent::Talent>,
}

impl Character {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn owner(&self) -> u64 {
        self.owner
    }
    pub(crate) fn get_attribute_by_id(&self, id: &str) -> Option<&attribute::Attribute> {
        let idu = id.to_uppercase();
        self.attributes.get(idu.as_str())
    }
    pub(crate) fn get_talent_by_id(&self, id: &str) -> Option<&talent::Talent> {
        let id_clean = talent::Talent::clean_talent_id(id);
        match id_clean {
            "ERROR" => None,
            _ => self.talents.get(id),
        }
    }
    pub(crate) fn ini(&self) -> initiative::Ini {
        let d = lib::dice::Dice::new(6);
        let ad = attribute::Attribute::default();
        let ini = (self
            .get_attribute_by_id(attribute::ATTRIBUTE_ID_MU)
            .unwrap_or(&ad)
            .value()
            + self
                .get_attribute_by_id(attribute::ATTRIBUTE_ID_GE)
                .unwrap_or(&ad)
                .value())
            / 2;
        initiative::Ini::new(ini, d.roll(), lib::modifier::Modifier::default())
    }
    pub(crate) fn check_talent(
        &self,
        id: &str,
        mod_value: u8,
        mod_op: lib::operator::Operator,
    ) -> String {
        let talent_id = talent::Talent::clean_talent_id(id);
        let default_talent = match talent::Talent::get_default_by_id(talent_id) {
            Some(t) => t,
            _ => {
                warn!("The talent with id \"{}\" could not be matched.", id);
                return format!("The talent \"{}\" does not exist.", id);
            }
        };
        let talent = match self.get_talent_by_id(talent_id) {
            Some(tal) => tal,
            _ => &default_talent,
        };
        let mut current_talent_val = talent.value();
        let d = attribute::Attribute::default();
        let attr1 = match self.get_attribute_by_id(talent.attribute_1_id()) {
            Some(attr) => attr,
            _ => &d,
        };
        let attr2 = match self.get_attribute_by_id(talent.attribute_2_id()) {
            Some(attr) => attr,
            _ => &d,
        };
        let attr3 = match self.get_attribute_by_id(talent.attribute_3_id()) {
            Some(attr) => attr,
            _ => &d,
        };
        let mut actual_attribute_value_1 = match self.get_attribute_by_id(talent.attribute_1_id()) {
            Some(attr) => attr.value(),
            _ => 0,
        };
        let mut actual_attribute_value_2 = match self.get_attribute_by_id(talent.attribute_2_id()) {
            Some(attr) => attr.value(),
            _ => 0,
        };
        let mut actual_attribute_value_3 = match self.get_attribute_by_id(talent.attribute_3_id()) {
            Some(attr) => attr.value(),
            _ => 0,
        };
        if mod_value > 0 && mod_op != lib::operator::Operator::NoP {
            if mod_op == lib::operator::Operator::Minus {
                actual_attribute_value_1 =
                    util::uint8::subtract_without_overflow(actual_attribute_value_1, mod_value);
                actual_attribute_value_2 =
                    util::uint8::subtract_without_overflow(actual_attribute_value_2, mod_value);
                actual_attribute_value_3 =
                    util::uint8::subtract_without_overflow(actual_attribute_value_3, mod_value);
            } else {
                actual_attribute_value_1 =
                    util::uint8::add_without_overflow(actual_attribute_value_1, mod_value);
                actual_attribute_value_2 =
                    util::uint8::add_without_overflow(actual_attribute_value_2, mod_value);
                actual_attribute_value_3 =
                    util::uint8::add_without_overflow(actual_attribute_value_3, mod_value);
            }
        }
        let r1 = check_talent_roll(attr1, actual_attribute_value_1, &mut current_talent_val);
        let r2 = check_talent_roll(attr2, actual_attribute_value_2, &mut current_talent_val);
        let r3 = check_talent_roll(attr3, actual_attribute_value_3, &mut current_talent_val);
        let is_success = r1.is_success() && r2.is_success() && r3.is_success();
        let is_critical_success = is_success
            && (r1.is_critical_success() || r2.is_critical_success() || r3.is_critical_success());
        let is_critical_failure = !is_success
            && (r1.is_critical_failure() || r2.is_critical_failure() || r3.is_critical_failure());
        let check = match mod_op {
            lib::operator::Operator::Minus => format!("{}(-{})", talent.name(), mod_value),
            lib::operator::Operator::Plus => format!("{}(+{})", talent.name(), mod_value),
            lib::operator::Operator::NoP => talent.name().to_string(),
        };
        let msg = if is_critical_success {
            util::localisation::get_text("dsa.character.is-critical-success")
        } else if is_success {
            util::localisation::get_text("dsa.character.is-success")
        } else if !is_success && !is_critical_failure && !is_critical_success {
            util::localisation::get_text("dsa.character.is-failure")
        } else if !is_success && !is_critical_success && is_critical_failure {
            util::localisation::get_text("dsa.character.is-critical-failure")
        } else {
            util::localisation::get_text("dsa.character.success-unknown")
        };
        let mut q = get_quality(current_talent_val);
        if is_critical_success {
            if r1.is_critical_success() {
                q = util::uint8::add_without_overflow(q, r1.crit_count());
            }
            if r2.is_critical_success() {
                q = util::uint8::add_without_overflow(q, r2.crit_count());
            }
            if r3.is_critical_success() {
                q = util::uint8::add_without_overflow(q, r3.crit_count());
            }
        }
        if is_success {
            format!(
                "{} **{} - {}{}{}!**\n{}\n{}\n{}",
                util::localisation::get_text("dsa.character.is-talent-check-success-emoji"),
                check,
                msg,
                util::localisation::get_text(
                    "dsa.character.is-talent-check-success-with-quality-level"
                ),
                q,
                r1,
                r2,
                r3
            )
        } else {
            format!(
                "{} **{} - {}**\n{}\n{}\n{}",
                util::localisation::get_text("dsa.character.is-talent-check-failure-emoji"),
                check,
                msg,
                r1,
                r2,
                r3
            )
        }
    }
}

fn get_quality(talent_left: u8) -> u8 {
    let q = talent_left / 3;
    info!(
        "Checking quality level: {} -> quality indicator {}",
        talent_left, q
    );
    if q == 0 {
        1
    } else if q >= 6 {
        6
    } else {
        q
    }
}

fn get_attribute_display(attribute: &attribute::Attribute, value: u8) -> String {
    match attribute.value().cmp(&value) {
        Ordering::Equal => format!("{} {}", attribute.name(), attribute.value()),
        Ordering::Less => format!(
            "{} {}(+{})",
            attribute.name(),
            attribute.value(),
            util::uint8::subtract_without_overflow(value, attribute.value())
        ),
        Ordering::Greater => format!(
            "{} {}(-{})",
            attribute.name(),
            attribute.value(),
            util::uint8::subtract_without_overflow(attribute.value(), value)
        ),
    }
}

fn check_talent_roll(
    attribute: &attribute::Attribute,
    attribute_val: u8,
    current_talent_val: &mut u8,
) -> checks::talent::TalentCheckResult {
    let d = lib::dice::Dice::new(20);
    let roll = d.roll();
    if roll == 1 {
        let crit_count = dsa::util::check_critical_success(attribute_val, 0);
        match crit_count {
            0 => {
                let s = format!(
                    "{} - {}: {},{}",
                    util::localisation::get_text("dsa.character.is-attribute-check-success-emoji"),
                    get_attribute_display(attribute, attribute_val),
                    roll,
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-success-check-failed"
                    )
                );
                return checks::talent::TalentCheckResult::new(
                    &s,
                    result::ResultType::Success,
                    crit_count,
                );
            }
            1 => {
                let s = format!(
                    "{} - {}: {}, {}",
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-success-emoji"
                    ),
                    get_attribute_display(attribute, attribute_val),
                    roll,
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-success-check-success"
                    )
                );
                return checks::talent::TalentCheckResult::new(
                    &s,
                    result::ResultType::CriticalSuccess,
                    crit_count,
                );
            }
            _ => {
                let s = format!(
                    "{} - {}: {}, {}{}",
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-success-emoji"
                    ),
                    get_attribute_display(attribute, attribute_val),
                    roll,
                    crit_count,
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-success-check-success-n-times"
                    )
                );
                return checks::talent::TalentCheckResult::new(
                    &s,
                    result::ResultType::CriticalSuccess,
                    crit_count,
                );
            }
        }
    }
    if roll == 20 {
        let crit_count = dsa::util::check_critical_failure(attribute_val, 0);
        match crit_count {
            0 => {
                let s = format!(
                    "{} - {}: {}, {}",
                    util::localisation::get_text("dsa.character.is-attribute-check-failure-emoji"),
                    get_attribute_display(attribute, attribute_val),
                    roll,
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-failure-check-success"
                    )
                );
                return checks::talent::TalentCheckResult::new(
                    &s,
                    result::ResultType::Failure,
                    crit_count,
                );
            }
            1 => {
                let s = format!(
                    "{} - {}: {}, {}",
                    util::localisation::get_text("dsa.character.is-attribute-check-failure-emoji"),
                    get_attribute_display(attribute, attribute_val),
                    roll,
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-failure-check-failure"
                    )
                );
                return checks::talent::TalentCheckResult::new(
                    &s,
                    result::ResultType::CriticalFailure,
                    crit_count,
                );
            }
            _ => {
                let s = format!(
                    "{} - {}: {}, {}{}",
                    util::localisation::get_text("dsa.character.is-attribute-check-failure-emoji"),
                    get_attribute_display(attribute, attribute_val),
                    roll,
                    crit_count,
                    util::localisation::get_text(
                        "dsa.character.is-attribute-check-critical-failure-check-failure-n-times"
                    )
                );
                return checks::talent::TalentCheckResult::new(
                    &s,
                    result::ResultType::CriticalFailure,
                    crit_count,
                );
            }
        }
    }
    if roll <= attribute_val {
        let s = format!(
            "{} - {}: {}",
            util::localisation::get_text("dsa.character.is-attribute-check-success-emoji"),
            get_attribute_display(attribute, attribute_val),
            roll
        );
        return checks::talent::TalentCheckResult::new(&s, result::ResultType::Success, 0);
    }
    if *current_talent_val < 1 {
        let s = format!(
            "{} - {}: {}",
            util::localisation::get_text("dsa.character.is-attribute-check-failure-emoji"),
            get_attribute_display(attribute, attribute_val),
            roll
        );
        return checks::talent::TalentCheckResult::new(&s, result::ResultType::Failure, 0);
    }
    let diff = util::uint8::subtract_without_overflow(roll, attribute_val);
    if *current_talent_val < diff {
        let s = format!(
            "{} - {}: {}: {}{}{}{}",
            util::localisation::get_text("dsa.character.is-attribute-check-failure-emoji"),
            get_attribute_display(attribute, attribute_val),
            roll,
            *current_talent_val,
            util::localisation::get_text(
                "dsa.character.is-attribute-check-failed-not-compensated.00"
            ),
            diff,
            util::localisation::get_text(
                "dsa.character.is-attribute-check-failed-not-compensated.01"
            )
        );
        checks::talent::TalentCheckResult::new(&s, result::ResultType::Failure, 0)
    } else {
        *current_talent_val = util::uint8::subtract_without_overflow(*current_talent_val, diff);
        let s = format!(
            "{} - {}: {}: {}{}{}{}",
            util::localisation::get_text(
                "dsa.character.is-attribute-check-failed-but-compensated-emoji"
            ),
            get_attribute_display(attribute, attribute_val),
            roll,
            diff,
            util::localisation::get_text(
                "dsa.character.is-attribute-check-failed-but-compensated.00"
            ),
            *current_talent_val,
            util::localisation::get_text(
                "dsa.character.is-attribute-check-failed-but-compensated.01"
            )
        );
        checks::talent::TalentCheckResult::new(&s, result::ResultType::Success, 0)
    }
}
