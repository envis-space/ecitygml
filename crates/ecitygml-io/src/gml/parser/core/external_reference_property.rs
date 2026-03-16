use crate::gml::parser::core::external_reference::GmlExternalReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlExternalReferenceProperty {
    #[serde(rename = "$value")]
    pub content: GmlExternalReference,
}
