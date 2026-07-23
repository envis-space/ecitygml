use crate::model::core::basic_types::DoubleBetween0And1Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GmlError(#[from] egml::Error),
    #[error(transparent)]
    DoubleBetween0And1(#[from] DoubleBetween0And1Error),

    #[error("the data for key `{0}` is not available")]
    ContainsNoMembers(String),
    #[error("the data for key `{0}` is not available")]
    ElementNotFound(String),
    #[error("the data for key `{0}` is not available")]
    InvalidLodName(String),
    #[error("transformation matrix expected 16 values, got {found}")]
    InvalidMatrixSize { found: usize },
}
