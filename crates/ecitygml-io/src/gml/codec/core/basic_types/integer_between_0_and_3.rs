use ecitygml_core::model::common::LevelOfDetail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlIntegerBetween0And3 {
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
}

impl From<LevelOfDetail> for GmlIntegerBetween0And3 {
    fn from(item: LevelOfDetail) -> Self {
        match item {
            LevelOfDetail::Zero => Self::Zero,
            LevelOfDetail::One => Self::One,
            LevelOfDetail::Two => Self::Two,
            LevelOfDetail::Three => Self::Three,
        }
    }
}

impl From<GmlIntegerBetween0And3> for LevelOfDetail {
    fn from(item: GmlIntegerBetween0And3) -> Self {
        match item {
            GmlIntegerBetween0And3::Zero => LevelOfDetail::Zero,
            GmlIntegerBetween0And3::One => LevelOfDetail::One,
            GmlIntegerBetween0And3::Two => LevelOfDetail::Two,
            GmlIntegerBetween0And3::Three => LevelOfDetail::Three,
        }
    }
}
