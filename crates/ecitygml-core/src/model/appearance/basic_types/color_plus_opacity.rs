use crate::model::appearance::basic_types::Color;
use crate::model::core::basic_types::{DoubleBetween0And1, DoubleBetween0And1Error};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPlusOpacity {
    color: Color,
    opacity: DoubleBetween0And1,
}

impl ColorPlusOpacity {
    pub fn new(color: Color, opacity: DoubleBetween0And1) -> Self {
        Self { color, opacity }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn opacity(&self) -> DoubleBetween0And1 {
        self.opacity
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ColorError {
    #[error("color component '{component}' is out of range: {source}")]
    OutOfRange {
        component: &'static str,
        #[source]
        source: DoubleBetween0And1Error,
    },
}
