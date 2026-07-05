use ecitygml_core::model::core::basic_types::DoubleBetween0And1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct GmlDoubleBetween0And1(pub f64);

impl TryFrom<GmlDoubleBetween0And1> for DoubleBetween0And1 {
    type Error = ecitygml_core::Error;

    fn try_from(gml: GmlDoubleBetween0And1) -> Result<Self, Self::Error> {
        Ok(DoubleBetween0And1::try_from(gml.0)?)
    }
}

impl From<DoubleBetween0And1> for GmlDoubleBetween0And1 {
    fn from(item: DoubleBetween0And1) -> Self {
        Self(item.value())
    }
}
