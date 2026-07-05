use ecitygml_core::model::appearance::enums::WrapMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub enum GmlWrapMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "wrap")]
    Wrap,
    #[serde(rename = "mirror")]
    Mirror,
    #[serde(rename = "clamp")]
    Clamp,
    #[serde(rename = "border")]
    Border,
}

impl From<GmlWrapMode> for WrapMode {
    fn from(item: GmlWrapMode) -> Self {
        match item {
            GmlWrapMode::None => WrapMode::None,
            GmlWrapMode::Wrap => WrapMode::Wrap,
            GmlWrapMode::Mirror => WrapMode::Mirror,
            GmlWrapMode::Clamp => WrapMode::Clamp,
            GmlWrapMode::Border => WrapMode::Border,
        }
    }
}

impl From<WrapMode> for GmlWrapMode {
    fn from(item: WrapMode) -> Self {
        match item {
            WrapMode::None => GmlWrapMode::None,
            WrapMode::Wrap => GmlWrapMode::Wrap,
            WrapMode::Mirror => GmlWrapMode::Mirror,
            WrapMode::Clamp => GmlWrapMode::Clamp,
            WrapMode::Border => GmlWrapMode::Border,
        }
    }
}
