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
            GmlTrafficDirectionValue::Forwards => TrafficDirectionValue::Forwards,
            GmlTrafficDirectionValue::Backwards => TrafficDirectionValue::Backwards,
            GmlTrafficDirectionValue::Both => TrafficDirectionValue::Both,
        }
    }
}

impl From<&TrafficDirectionValue> for GmlTrafficDirectionValue {
    fn from(item: &TrafficDirectionValue) -> Self {
        match item {
            TrafficDirectionValue::Forwards => GmlTrafficDirectionValue::Forwards,
            TrafficDirectionValue::Backwards => GmlTrafficDirectionValue::Backwards,
            TrafficDirectionValue::Both => GmlTrafficDirectionValue::Both,
        }
    }
}
