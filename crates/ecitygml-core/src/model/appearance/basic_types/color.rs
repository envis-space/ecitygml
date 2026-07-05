use crate::model::core::basic_types::DoubleBetween0And1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: DoubleBetween0And1,
    green: DoubleBetween0And1,
    blue: DoubleBetween0And1,
}

impl Color {
    pub fn new(
        red: DoubleBetween0And1,
        green: DoubleBetween0And1,
        blue: DoubleBetween0And1,
    ) -> Self {
        Self { red, green, blue }
    }

    pub fn red(&self) -> DoubleBetween0And1 {
        self.red
    }

    pub fn green(&self) -> DoubleBetween0And1 {
        self.green
    }

    pub fn blue(&self) -> DoubleBetween0And1 {
        self.blue
    }
}
