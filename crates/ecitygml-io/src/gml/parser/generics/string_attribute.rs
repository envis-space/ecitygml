use crate::Error;
use crate::Error::AttributeWithoutName;
use ecitygml_core::model::generics::StringAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GmlStringAttribute {
    pub name: String,
    pub value: String,
}

impl TryFrom<GmlStringAttribute> for StringAttribute {
    type Error = Error;

    fn try_from(item: GmlStringAttribute) -> Result<Self, Self::Error> {
        if item.name.is_empty() {
            return Err(AttributeWithoutName("string attribute".to_string()));
        }

        Ok(Self {
            name: item.name,
            value: item.value,
        })
    }
}
