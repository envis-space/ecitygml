use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcitygmlError(#[from] ecitygml_core::Error),
}
