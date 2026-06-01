use ecitygml_core::model::core::RelativeToWater;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub enum GmlRelativeToWater {
    #[serde(rename = "entirelyAboveWaterSurface")]
    EntirelyAboveWaterSurface,

    #[serde(rename = "substantiallyAboveWaterSurface")]
    SubstantiallyAboveWaterSurface,

    #[serde(rename = "substantiallyAboveAndBelowWaterSurface")]
    SubstantiallyAboveAndBelowWaterSurface,

    #[serde(rename = "substantiallyBelowWaterSurface")]
    SubstantiallyBelowWaterSurface,

    #[serde(rename = "entirelyBelowWaterSurface")]
    EntirelyBelowWaterSurface,

    #[serde(rename = "temporarilyAboveAndBelowWaterSurface")]
    TemporarilyAboveAndBelowWaterSurface,
}

impl From<GmlRelativeToWater> for RelativeToWater {
    fn from(item: GmlRelativeToWater) -> Self {
        match item {
            GmlRelativeToWater::EntirelyAboveWaterSurface => Self::EntirelyAboveWaterSurface,
            GmlRelativeToWater::SubstantiallyAboveWaterSurface => {
                Self::SubstantiallyAboveWaterSurface
            }
            GmlRelativeToWater::SubstantiallyAboveAndBelowWaterSurface => {
                Self::SubstantiallyAboveAndBelowWaterSurface
            }
            GmlRelativeToWater::SubstantiallyBelowWaterSurface => {
                Self::SubstantiallyBelowWaterSurface
            }
            GmlRelativeToWater::EntirelyBelowWaterSurface => Self::EntirelyBelowWaterSurface,
            GmlRelativeToWater::TemporarilyAboveAndBelowWaterSurface => {
                Self::TemporarilyAboveAndBelowWaterSurface
            }
        }
    }
}

impl From<RelativeToWater> for GmlRelativeToWater {
    fn from(item: RelativeToWater) -> Self {
        match item {
            RelativeToWater::EntirelyAboveWaterSurface => Self::EntirelyAboveWaterSurface,
            RelativeToWater::SubstantiallyAboveWaterSurface => Self::SubstantiallyAboveWaterSurface,
            RelativeToWater::SubstantiallyAboveAndBelowWaterSurface => {
                Self::SubstantiallyAboveAndBelowWaterSurface
            }
            RelativeToWater::SubstantiallyBelowWaterSurface => Self::SubstantiallyBelowWaterSurface,
            RelativeToWater::EntirelyBelowWaterSurface => Self::EntirelyBelowWaterSurface,
            RelativeToWater::TemporarilyAboveAndBelowWaterSurface => {
                Self::TemporarilyAboveAndBelowWaterSurface
            }
        }
    }
}
