use crate::dsa::result::ResultType;
use std::fmt::Debug;
use std::fmt::Display;

pub(crate) mod talent {

  use crate::dsa::checks::CheckResult;
  use crate::dsa::result::ResultType;
  use std::fmt::*;

  #[derive(Debug, Default)]
  pub(crate) struct TalentCheckResult {
    pub(self) message: String,
    pub(self) result: ResultType,
    pub(self) crit_count: u8,
  }

  impl CheckResult for TalentCheckResult {
    fn message(&self) -> &str {
      &self.message
    }
    fn result(&self) -> ResultType {
      self.result
    }
    fn crit_count(&self) -> u8 {
      self.crit_count
    }
    fn new(message: &str, result: ResultType, crit_count: u8) -> Self {
      Self {
        message: String::from(message),
        result,
        crit_count,
      }
    }
  }

  impl Display for TalentCheckResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}", self.message())
    }
  }
}
pub(crate) mod spell {}

trait Check {}
pub(crate) trait CheckResult: Display + Debug + Default {
  fn message(&self) -> &str;
  fn result(&self) -> ResultType;
  fn crit_count(&self) -> u8;
  fn new(message: &str, result: ResultType, crit_count: u8) -> Self;
  fn is_success(&self) -> bool {
    match self.result() {
      ResultType::CriticalSuccess => true,
      ResultType::Success => true,
      _ => false,
    }
  }
  fn is_critical_success(&self) -> bool {
    match self.result() {
      ResultType::CriticalSuccess => true,
      _ => false,
    }
  }
  fn is_failure(&self) -> bool {
    match self.result() {
      ResultType::CriticalFailure => true,
      ResultType::Failure => true,
      _ => false,
    }
  }
  fn is_critical_failure(&self) -> bool {
    match self.result() {
      ResultType::CriticalFailure => true,
      _ => false,
    }
  }
}
