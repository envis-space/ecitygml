use ecitygml_core::model::core::RelativeToTerrain;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub enum GmlRelativeToTerrain {
    #[serde(rename = "entirelyAboveTerrain")]
    EntirelyAboveTerrain,

    #[serde(rename = "substantiallyAboveTerrain")]
    SubstantiallyAboveTerrain,

    #[serde(rename = "substantiallyAboveAndBelowTerrain")]
    SubstantiallyAboveAndBelowTerrain,

    #[serde(rename = "substantiallyBelowTerrain")]
    SubstantiallyBelowTerrain,

    #[serde(rename = "entirelyBelowTerrain")]
    EntirelyBelowTerrain,
}

impl From<GmlRelativeToTerrain> for RelativeToTerrain {
    fn from(item: GmlRelativeToTerrain) -> Self {
        match item {
            GmlRelativeToTerrain::EntirelyAboveTerrain => Self::EntirelyAboveTerrain,
            GmlRelativeToTerrain::SubstantiallyAboveTerrain => Self::SubstantiallyAboveTerrain,
            GmlRelativeToTerrain::SubstantiallyAboveAndBelowTerrain => {
                Self::SubstantiallyAboveAndBelowTerrain
            }
            GmlRelativeToTerrain::SubstantiallyBelowTerrain => Self::SubstantiallyBelowTerrain,
            GmlRelativeToTerrain::EntirelyBelowTerrain => Self::EntirelyBelowTerrain,
        }
    }
}
