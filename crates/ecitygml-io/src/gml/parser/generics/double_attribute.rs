use crate::Error;
use crate::Error::AttributeWithoutName;
use ecitygml_core::model::generics::DoubleAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlDoubleAttribute {
    pub name: String,
    pub value: f64,
}

impl TryFrom<GmlDoubleAttribute> for DoubleAttribute {
    type Error = Error;

    fn try_from(item: GmlDoubleAttribute) -> Result<Self, Self::Error> {
        if item.name.is_empty() {
            return Err(AttributeWithoutName("string attribute".to_string()));
        }

        Ok(Self {
            name: item.name,
            value: item.value,
        })
    }
}
