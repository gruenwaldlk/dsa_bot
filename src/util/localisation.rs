use crate::CTX;
use json_gettext::JSONGetTextValue;
use log::debug;
use log::error;
use std::env;

pub(crate) fn get_text(key: &str) -> String {
  let s = format!(
    "{}",
    get_text!(CTX, curr_lang(), key).unwrap_or_else(|| JSONGetTextValue::from_str("MISSING"))
  );
  if &s == "MISSING" {
    error!(
      "Missing translation - Language: {}, Key: \"{}\"",
      curr_lang(),
      key
    );
  }
  s
}

pub(crate) fn curr_lang() -> String {
  let current_language = env::var("CURRENT_LANGUAGE").expect("Expected a token in the environment");
  debug!("Current language: {}", current_language);
  current_language
}
