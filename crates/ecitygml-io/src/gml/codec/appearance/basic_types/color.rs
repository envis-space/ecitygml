use ecitygml_core::model::appearance::basic_types::Color;
use ecitygml_core::model::core::basic_types::DoubleBetween0And1;
use egml::io::util::serde_helpers::{
    deserialize_space_separated_f64_3, serialize_space_separated_f64_3,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct GmlColor(
    #[serde(
        deserialize_with = "deserialize_space_separated_f64_3",
        serialize_with = "serialize_space_separated_f64_3"
    )]
    pub [f64; 3],
);

impl TryFrom<GmlColor> for Color {
    type Error = ecitygml_core::Error;

    fn try_from(gml: GmlColor) -> Result<Self, Self::Error> {
        let [r, g, b] = gml.0;
        Ok(Color::new(
            DoubleBetween0And1::try_from(r)?,
            DoubleBetween0And1::try_from(g)?,
            DoubleBetween0And1::try_from(b)?,
        ))
    }
}

impl From<Color> for GmlColor {
    fn from(item: Color) -> Self {
        Self([
            item.red().value(),
            item.green().value(),
            item.blue().value(),
        ])
    }
}
