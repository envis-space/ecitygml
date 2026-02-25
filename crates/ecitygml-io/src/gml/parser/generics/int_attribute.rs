use crate::Error;
use crate::Error::AttributeWithoutName;
use ecitygml_core::model::generics::IntAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GmlIntAttribute {
    pub name: String,
    pub value: i64,
}

impl TryFrom<GmlIntAttribute> for IntAttribute {
    type Error = Error;

    fn try_from(item: GmlIntAttribute) -> Result<Self, Self::Error> {
        if item.name.is_empty() {
            return Err(AttributeWithoutName("string attribute".to_string()));
        }

        Ok(Self {
            name: item.name,
            value: item.value,
        })
    }
}
