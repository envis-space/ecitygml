use ecitygml_core::model::core::SpaceType;
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
            GmlSpaceType::Closed => Self::Closed,
            GmlSpaceType::Open => Self::Open,
            GmlSpaceType::SemiOpen => Self::SemiOpen,
        }
    }
}
