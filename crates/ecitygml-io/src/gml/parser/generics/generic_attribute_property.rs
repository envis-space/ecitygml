use crate::gml::parser::generics::GmlGenericAttributeKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlGenericAttributeProperty {
    #[serde(rename = "$value")]
    pub content: GmlGenericAttributeKind,
}
