use crate::gml::codec::core::external_reference::GmlExternalReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlExternalReferenceProperty {
    #[serde(rename = "ExternalReference")]
    pub content: GmlExternalReference,
}
