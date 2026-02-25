mod auto;
mod error;
mod format;
pub mod gml;

#[doc(inline)]
pub use crate::gml::read::GmlReader;

#[doc(inline)]
pub use crate::error::Error;

#[doc(inline)]
pub use crate::format::CitygmlFormat;

#[doc(inline)]
pub use crate::gml::FILE_EXTENSION_GML_FORMAT;

#[doc(inline)]
pub use crate::gml::FILE_EXTENSION_XML_FORMAT;

#[doc(inline)]
pub use crate::gml::FILE_EXTENSION_GML_ZST_FORMAT;

#[doc(inline)]
pub use crate::gml::FILE_EXTENSION_GML_GZ_FORMAT;
