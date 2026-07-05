use ecitygml_core::model::appearance::GeometryReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlGeometryReference {
    #[serde(rename = "$text")]
    pub href: String,
}

impl From<GmlGeometryReference> for GeometryReference {
    fn from(gml: GmlGeometryReference) -> Self {
        Self::new(gml.href)
    }
}

impl From<&GeometryReference> for GmlGeometryReference {
    fn from(item: &GeometryReference) -> Self {
        Self {
            href: item.href.clone(),
        }
    }
}
