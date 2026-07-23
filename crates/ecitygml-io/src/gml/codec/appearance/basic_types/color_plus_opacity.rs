use ecitygml_core::model::appearance::basic_types::{Color, ColorPlusOpacity};
use ecitygml_core::model::core::basic_types::DoubleBetween0And1;
use egml::io::util::serde_helpers::{
    deserialize_space_separated_f64_4, serialize_space_separated_f64_4,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct GmlColorPlusOpacity(
    #[serde(
        deserialize_with = "deserialize_space_separated_f64_4",
        serialize_with = "serialize_space_separated_f64_4"
    )]
    pub [f64; 4],
);

impl TryFrom<GmlColorPlusOpacity> for ColorPlusOpacity {
    type Error = ecitygml_core::Error;

    fn try_from(gml: GmlColorPlusOpacity) -> Result<Self, Self::Error> {
        let [r, g, b, a] = gml.0;
        Ok(ColorPlusOpacity::new(
            Color::new(
                DoubleBetween0And1::try_from(r)?,
                DoubleBetween0And1::try_from(g)?,
                DoubleBetween0And1::try_from(b)?,
            ),
            DoubleBetween0And1::try_from(a)?,
        ))
    }
}

impl From<ColorPlusOpacity> for GmlColorPlusOpacity {
    fn from(item: ColorPlusOpacity) -> Self {
        Self([
            item.color().red().value(),
            item.color().green().value(),
            item.color().blue().value(),
            item.opacity().value(),
        ])
    }
}
