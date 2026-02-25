use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcitygmlError(#[from] ecitygml_core::Error),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    #[error(transparent)]
    StdStrUtf(#[from] std::str::Utf8Error),
    #[error(transparent)]
    QuickXmlError(#[from] quick_xml::Error),
    #[error(transparent)]
    QuickXmlDeError(#[from] quick_xml::DeError),
    #[error(transparent)]
    QuickXmlSeError(#[from] quick_xml::SeError),
    #[error(transparent)]
    EgmlError(#[from] egml::Error),
    #[error(transparent)]
    EgmlIoError(#[from] egml::io::Error),

    #[error("file extension is invalid")]
    NoFileExtension(),
    #[error("file extension `{0}` is invalid")]
    InvalidFileExtension(String),
    #[error("the data for key `{0}` is not available")]
    ElementNotFound(String),
    #[error("the data for key `{0}` is not available")]
    UnknownElementNode(String),
    #[error("attribute has no name: `{0}")]
    AttributeWithoutName(String),
    #[error("file extension is invalid")]
    NoFileName(),
}
