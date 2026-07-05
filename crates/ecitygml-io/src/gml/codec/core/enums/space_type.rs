use ecitygml_core::model::core::enums::SpaceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum GmlSpaceType {
    #[serde(rename = "closed")]
    #[default]
    Closed,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "semiOpen")]
    SemiOpen,
}

impl From<GmlSpaceType> for SpaceType {
    fn from(item: GmlSpaceType) -> Self {
        match item {
            GmlSpaceType::Closed => SpaceType::Closed,
            GmlSpaceType::Open => SpaceType::Open,
            GmlSpaceType::SemiOpen => SpaceType::SemiOpen,
        }
    }
}

impl From<SpaceType> for GmlSpaceType {
    fn from(item: SpaceType) -> Self {
        match item {
            SpaceType::Closed => GmlSpaceType::Closed,
            SpaceType::Open => GmlSpaceType::Open,
            SpaceType::SemiOpen => GmlSpaceType::SemiOpen,
        }
    }
}
