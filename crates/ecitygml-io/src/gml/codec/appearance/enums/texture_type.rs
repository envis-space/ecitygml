use ecitygml_core::model::appearance::enums::TextureType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub enum GmlTextureType {
    #[serde(rename = "specific")]
    Specific,
    #[serde(rename = "typical")]
    Typical,
    #[serde(rename = "unknown")]
    Unknown,
}

impl From<GmlTextureType> for TextureType {
    fn from(item: GmlTextureType) -> Self {
        match item {
            GmlTextureType::Specific => TextureType::Specific,
            GmlTextureType::Typical => TextureType::Typical,
            GmlTextureType::Unknown => TextureType::Unknown,
        }
    }
}

impl From<TextureType> for GmlTextureType {
    fn from(item: TextureType) -> Self {
        match item {
            TextureType::Specific => GmlTextureType::Specific,
            TextureType::Typical => GmlTextureType::Typical,
            TextureType::Unknown => GmlTextureType::Unknown,
        }
    }
}
