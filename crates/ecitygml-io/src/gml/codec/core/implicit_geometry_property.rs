use crate::gml::codec::core::GmlImplicitGeometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlImplicitGeometryProperty {
    #[serde(rename = "$value")]
    pub implicit_geometry: GmlImplicitGeometry,
}
