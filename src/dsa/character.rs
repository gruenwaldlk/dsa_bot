use crate::dsa::attribute::*;
use crate::dsa::checks::talent::TalentCheckResult;
use crate::dsa::checks::CheckResult;
use crate::dsa::talent::*;
use crate::dsa::ResultType;
use crate::dsa::*;
use crate::lib::dice::Dice;
use crate::lib::*;
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
    pub(self) attributes: HashMap<String, Attribute>,
    pub(self) talents: HashMap<String, Talent>,
    pub(self) spells: HashMap<String, Talent>,
}

impl Character {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn owner(&self) -> u64 {
        self.owner
    }
    pub(crate) fn get_attribute_by_id(&self, id: &str) -> Option<&Attribute> {
        let idu = id.to_uppercase();
        self.attributes.get(idu.as_str())
    }
    pub(crate) fn get_talent_by_id(&self, id: &str) -> Option<&Talent> {
        let id_clean = Talent::clean_talent_id(id);
        match id_clean {
            "ERROR" => None,
            _ => self.talents.get(id),
        }
    }
    pub(crate) fn ini(&self) -> u8 {
        let d = Dice::new(6);
        let ad = Attribute::default();
        let ini = (self
            .get_attribute_by_id(ATTRIBUTE_ID_MU)
            .unwrap_or(&ad)
            .value()
            + self
                .get_attribute_by_id(ATTRIBUTE_ID_GE)
                .unwrap_or(&ad)
                .value())
            / 2;
        ini + d.roll()
    }
    pub(crate) fn check_talent(&self, id: &str, mod_value: u8, mod_op: Operator) -> String {
        let talent_id = Talent::clean_talent_id(id);
        let default_talent = match Talent::get_default_by_id(talent_id) {
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
        let d = Attribute::default();
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
        if mod_value > 0 && mod_op != Operator::NoP {
            if mod_op == Operator::Minus {
                actual_attribute_value_1 =
                    subtract_without_overflow(actual_attribute_value_1, mod_value);
                actual_attribute_value_2 =
                    subtract_without_overflow(actual_attribute_value_2, mod_value);
                actual_attribute_value_3 =
                    subtract_without_overflow(actual_attribute_value_3, mod_value);
            } else {
                actual_attribute_value_1 =
                    add_without_overflow(actual_attribute_value_1, mod_value);
                actual_attribute_value_2 =
                    add_without_overflow(actual_attribute_value_2, mod_value);
                actual_attribute_value_3 =
                    add_without_overflow(actual_attribute_value_3, mod_value);
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
            Operator::Minus => format!("{}(-{})", talent.name(), mod_value),
            Operator::Plus => format!("{}(+{})", talent.name(), mod_value),
            Operator::NoP => talent.name().to_string(),
        };
        let msg = if is_critical_success {
            "Critical success"
        } else if is_success {
            "Success"
        } else if !is_success && !is_critical_failure && !is_critical_success {
            "Failure!"
        } else if !is_success && !is_critical_success && is_critical_failure {
            "Critical Failure!"
        } else {
            "Beep ... bop ... I'm not sure what happened, sorry!"
        };
        let mut q = get_quality(current_talent_val);
        if is_critical_success {
            if r1.is_critical_success() {
                q = add_without_overflow(q, r1.crit_count());
            }
            if r2.is_critical_success() {
                q = add_without_overflow(q, r2.crit_count());
            }
            if r3.is_critical_success() {
                q = add_without_overflow(q, r3.crit_count());
            }
        }
        if is_success {
            format!(
                ":white_check_mark: **{} - {} with quality level {}!**\n{}\n{}\n{}",
                check, msg, q, r1, r2, r3
            )
        } else {
            format!(":x: **{} - {}**\n{}\n{}\n{}", check, msg, r1, r2, r3)
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

fn get_attribute_display(attribute: &Attribute, value: u8) -> String {
    match attribute.value().cmp(&value) {
        Ordering::Equal => format!("{} {}", attribute.name(), attribute.value()),
        Ordering::Less => format!(
            "{} {}(+{})",
            attribute.name(),
            attribute.value(),
            subtract_without_overflow(value, attribute.value())
        ),
        Ordering::Greater => format!(
            "{} {}(-{})",
            attribute.name(),
            attribute.value(),
            subtract_without_overflow(attribute.value(), value)
        ),
    }
}

fn check_talent_roll(
    attribute: &Attribute,
    attribute_val: u8,
    current_talent_val: &mut u8,
) -> TalentCheckResult {
    let d = Dice::new(20);
    let roll = d.roll();
    if roll == 1 {
        let crit_count = check_critical_success(attribute_val, 0);
        if crit_count > 0 {
            let s = format!(
                ":partying_face: - {}: {}, crit check succeded {} time(-s)",
                get_attribute_display(attribute, attribute_val),
                roll,
                crit_count
            );
            return TalentCheckResult::new(&s, ResultType::CriticalSuccess, crit_count);
        } else {
            let s = format!(
                ":green_circle: - {}: {}, crit check failed.",
                get_attribute_display(attribute, attribute_val),
                roll
            );
            return TalentCheckResult::new(&s, ResultType::Success, crit_count);
        }
    }
    if roll == 20 {
        let crit_count = check_critical_failure(attribute_val, 0);
        if crit_count > 0 {
            let s = format!(
                ":skull: - {}: {}, crit check failed {} time(-s)",
                get_attribute_display(attribute, attribute_val),
                roll,
                crit_count
            );
            return TalentCheckResult::new(&s, ResultType::CriticalFailure, 1);
        } else {
            let s = format!(
                ":red_circle: - {}: {}, crit check succeeded.",
                get_attribute_display(attribute, attribute_val),
                roll
            );
            return TalentCheckResult::new(&s, ResultType::Failure, 0);
        }
    }
    if roll <= attribute_val {
        let s = format!(
            ":green_circle: - {}: {}",
            get_attribute_display(attribute, attribute_val),
            roll
        );
        return TalentCheckResult::new(&s, ResultType::Success, 0);
    }
    if *current_talent_val < 1 {
        let s = format!(
            ":red_circle: - {}: {}",
            get_attribute_display(attribute, attribute_val),
            roll
        );
        return TalentCheckResult::new(&s, ResultType::Failure, 0);
    }
    let diff = subtract_without_overflow(roll, attribute_val);
    if *current_talent_val < diff {
        let s = format!(
            ":red_circle: - {}: {}: {} points left, but {} missing.",
            get_attribute_display(attribute, attribute_val),
            roll,
            *current_talent_val,
            diff
        );
        TalentCheckResult::new(&s, ResultType::Failure, 0)
    } else {
        *current_talent_val = subtract_without_overflow(*current_talent_val, diff);
        let s = format!(
            ":orange_circle: - {}: {}: {} points used, {} left.",
            get_attribute_display(attribute, attribute_val),
            roll,
            diff,
            *current_talent_val
        );
        TalentCheckResult::new(&s, ResultType::Success, 0)
    }
}
