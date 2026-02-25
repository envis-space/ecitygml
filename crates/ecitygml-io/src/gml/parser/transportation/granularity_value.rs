use ecitygml_core::model::transportation::GranularityValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum GmlGranularityValue {
    #[serde(rename = "lane")]
    #[default]
    Lane,

    #[serde(rename = "way")]
    Way,
}

impl From<GmlGranularityValue> for GranularityValue {
    fn from(item: GmlGranularityValue) -> Self {
        match item {
            GmlGranularityValue::Lane => Self::Lane,
            GmlGranularityValue::Way => Self::Way,
        }
    }
}
