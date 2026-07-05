use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoubleBetween0And1(f64);

impl DoubleBetween0And1 {
    pub fn new(value: f64) -> Result<Self, DoubleBetween0And1Error> {
        if !(0.0..=1.0).contains(&value) {
            return Err(DoubleBetween0And1Error(value));
        }
        Ok(Self(value))
    }

    /// # Safety
    /// Caller must ensure the value is in `[0.0, 1.0]`.
    pub fn new_unchecked(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Error, Debug, PartialEq)]
#[error("value {0} is out of range [0.0, 1.0]")]
pub struct DoubleBetween0And1Error(pub f64);

impl From<DoubleBetween0And1> for f64 {
    fn from(x: DoubleBetween0And1) -> Self {
        x.0
    }
}

impl TryFrom<f64> for DoubleBetween0And1 {
    type Error = DoubleBetween0And1Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
