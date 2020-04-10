use crate::dsa::ResultType;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Talent {
    pub(self) name: String,
    pub(self) attribute_1_id: String,
    pub(self) attribute_2_id: String,
    pub(self) attribute_3_id: String,
    pub(self) value: u8,
}

impl Talent {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn attribute_1_id(&self) -> &str {
        &self.attribute_1_id
    }
    pub(crate) fn attribute_2_id(&self) -> &str {
        &self.attribute_2_id
    }
    pub(crate) fn attribute_3_id(&self) -> &str {
        &self.attribute_3_id
    }
    pub(crate) fn value(&self) -> u8 {
        self.value
    }
}

pub(crate) struct TalentCheckResult {
    pub(self) message: String,
    pub(self) result: ResultType,
    pub(self) crit_count: u8,
}

impl TalentCheckResult {
    pub(crate) fn message(&self) -> &str {
        &self.message
    }
    pub(crate) fn result(&self) -> ResultType {
        self.result
    }
    pub(crate) fn crit_count(&self) -> u8 {
        self.crit_count
    }
    pub(crate) fn new(message: &str, result: ResultType, crit_count: u8) -> Self {
        Self {
            message: String::from(message),
            result,
            crit_count,
        }
    }
    pub(crate) fn is_success(&self) -> bool {
        match self.result() {
            ResultType::CriticalSuccess => true,
            ResultType::Success => true,
            _ => false,
        }
    }
    pub(crate) fn is_critical_success(&self) -> bool {
        match self.result() {
            ResultType::CriticalSuccess => true,
            _ => false,
        }
    }
    pub(crate) fn is_failure(&self) -> bool {
        match self.result() {
            ResultType::CriticalFailure => true,
            ResultType::Failure => true,
            _ => false,
        }
    }
    pub(crate) fn is_critical_failure(&self) -> bool {
        match self.result() {
            ResultType::CriticalFailure => true,
            _ => false,
        }
    }
}
impl Display for TalentCheckResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message())
    }
}
