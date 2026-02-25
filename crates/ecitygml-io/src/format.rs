use crate::gml::FILE_EXTENSION_GML_GZ_FORMAT;
use crate::{FILE_EXTENSION_GML_FORMAT, FILE_EXTENSION_GML_ZST_FORMAT, FILE_EXTENSION_XML_FORMAT};
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CitygmlFormat {
    Gml,
    GmlZst,
    GmlGz,
}

impl CitygmlFormat {
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let path_str = path.as_ref().file_name()?.to_string_lossy().to_lowercase();

        match path_str {
            s if s.ends_with(FILE_EXTENSION_GML_FORMAT) => Some(Self::Gml),
            s if s.ends_with(FILE_EXTENSION_XML_FORMAT) => Some(Self::Gml),
            s if s.ends_with(FILE_EXTENSION_GML_GZ_FORMAT) => Some(Self::GmlGz),
            s if s.ends_with(FILE_EXTENSION_GML_ZST_FORMAT) => Some(Self::GmlZst),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Gml => FILE_EXTENSION_GML_FORMAT,
            Self::GmlZst => FILE_EXTENSION_GML_ZST_FORMAT,
            Self::GmlGz => FILE_EXTENSION_GML_GZ_FORMAT,
        }
    }

    pub fn is_supported_point_cloud_format(path: impl AsRef<Path>) -> bool {
        if !path.as_ref().is_file() {
            return false;
        }

        Self::from_path(&path).is_some()
    }
}
