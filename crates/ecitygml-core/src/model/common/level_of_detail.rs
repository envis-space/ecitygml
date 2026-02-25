use crate::Error;
use crate::Error::InvalidLodName;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, EnumIter, Ord, PartialOrd, Display)]
pub enum LevelOfDetail {
    Zero,
    One,
    Two,
    Three,
}

impl LevelOfDetail {
    pub fn as_index(self) -> usize {
        self as usize
    }
}

impl TryFrom<u8> for LevelOfDetail {
    type Error = Error;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(LevelOfDetail::Zero),
            1 => Ok(LevelOfDetail::One),
            2 => Ok(LevelOfDetail::Two),
            3 => Ok(LevelOfDetail::Three),
            _ => Err(InvalidLodName(item.to_string())),
        }
    }
}
