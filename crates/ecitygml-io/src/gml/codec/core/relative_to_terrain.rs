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
            GmlRelativeToTerrain::EntirelyAboveTerrain => RelativeToTerrain::EntirelyAboveTerrain,
            GmlRelativeToTerrain::SubstantiallyAboveTerrain => {
                RelativeToTerrain::SubstantiallyAboveTerrain
            }
            GmlRelativeToTerrain::SubstantiallyAboveAndBelowTerrain => {
                RelativeToTerrain::SubstantiallyAboveAndBelowTerrain
            }
            GmlRelativeToTerrain::SubstantiallyBelowTerrain => {
                RelativeToTerrain::SubstantiallyBelowTerrain
            }
            GmlRelativeToTerrain::EntirelyBelowTerrain => RelativeToTerrain::EntirelyBelowTerrain,
        }
    }
}

impl From<RelativeToTerrain> for GmlRelativeToTerrain {
    fn from(item: RelativeToTerrain) -> Self {
        match item {
            RelativeToTerrain::EntirelyAboveTerrain => GmlRelativeToTerrain::EntirelyAboveTerrain,
            RelativeToTerrain::SubstantiallyAboveTerrain => {
                GmlRelativeToTerrain::SubstantiallyAboveTerrain
            }
            RelativeToTerrain::SubstantiallyAboveAndBelowTerrain => {
                GmlRelativeToTerrain::SubstantiallyAboveAndBelowTerrain
            }
            RelativeToTerrain::SubstantiallyBelowTerrain => {
                GmlRelativeToTerrain::SubstantiallyBelowTerrain
            }
            RelativeToTerrain::EntirelyBelowTerrain => GmlRelativeToTerrain::EntirelyBelowTerrain,
        }
    }
}
