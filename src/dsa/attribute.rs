use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Attribute {
    pub(self) name: String,
    pub(self) value: u8,
}

impl Attribute {
    pub(crate) fn new(name: &str, value: u8) -> Attribute {
        Attribute {
            name: String::from(name),
            value,
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn value(&self) -> u8 {
        self.value
    }
}
