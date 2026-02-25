use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GmlError(#[from] egml::Error),

    #[error("the data for key `{0}` is not available")]
    ContainsNoMembers(String),
    #[error("the data for key `{0}` is not available")]
    ElementNotFound(String),
    #[error("the data for key `{0}` is not available")]
    InvalidLodName(String),
}
