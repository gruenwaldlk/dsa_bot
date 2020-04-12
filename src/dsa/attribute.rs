use serde::Deserialize;
use serde::Serialize;

pub(crate) const ATTRIBUTE_ID_MU: &str = "MU";
pub(crate) const ATTRIBUTE_ID_KL: &str = "KL";
pub(crate) const ATTRIBUTE_ID_IN: &str = "IN";
pub(crate) const ATTRIBUTE_ID_CH: &str = "CH";
pub(crate) const ATTRIBUTE_ID_FF: &str = "FF";
pub(crate) const ATTRIBUTE_ID_GE: &str = "GE";
pub(crate) const ATTRIBUTE_ID_KO: &str = "KO";
pub(crate) const ATTRIBUTE_ID_KK: &str = "KK";

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Attribute {
    pub(self) name: String,
    pub(self) value: u8,
}

impl Attribute {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn value(&self) -> u8 {
        self.value
    }
}
