use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub(crate) enum SpellType {
  Spell,
  Liturgy,
}
