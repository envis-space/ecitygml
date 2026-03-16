use crate::Error;
use crate::Error::AttributeWithoutName;
use ecitygml_core::model::generics::IntAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GmlIntAttribute {
    #[serde(rename(serialize = "gen:name", deserialize = "name"))]
    pub name: String,
    #[serde(rename(serialize = "gen:value", deserialize = "value"))]
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

impl From<&IntAttribute> for GmlIntAttribute {
    fn from(attr: &IntAttribute) -> Self {
        Self {
            name: attr.name.clone(),
            value: attr.value,
        }
    }
}
