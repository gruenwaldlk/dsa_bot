#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum ResultType {
  Failure,
  CriticalFailure,
  Success,
  CriticalSuccess,
}
impl Default for ResultType {
  fn default() -> Self {
    ResultType::Failure
  }
}
