use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_MU: &str = "MU";
#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_KL: &str = "KL";
#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_IN: &str = "IN";
#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_CH: &str = "CH";
#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_FF: &str = "FF";
#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_GE: &str = "GE";
#[allow(dead_code)]
pub(crate) const ATTRIBUTE_ID_KO: &str = "KO";
#[allow(dead_code)]
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
