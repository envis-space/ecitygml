use ecitygml_core::model::transportation::TrafficDirectionValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum GmlTrafficDirectionValue {
    #[serde(rename = "forwards")]
    #[default]
    Forwards,

    #[serde(rename = "backwards")]
    Backwards,

    #[serde(rename = "both")]
    Both,
}

impl From<GmlTrafficDirectionValue> for TrafficDirectionValue {
    fn from(item: GmlTrafficDirectionValue) -> Self {
        match item {
            GmlTrafficDirectionValue::Forwards => Self::Forwards,
            GmlTrafficDirectionValue::Backwards => Self::Backwards,
            GmlTrafficDirectionValue::Both => Self::Both,
        }
    }
}
