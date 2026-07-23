use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `ImplicitGeometry_mimeType.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/core/2.0/ImplicitGeometry_mimeType.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dImplicitGeometryMimeTypeValue {
    Vrml97,
    N3dsMax,
    AutocadDxfApplicationDxf,
    AutocadDxfApplicationXAutocad,
    AutocadDxfApplicationXDxf,
    AutocadDwg,
    Shockwave3d,
    X3dModelX3dXml,
    X3dModelX3dBinary,
    X3dModelX3dVrml,
    GifImages,
    JpegJpgImages,
    PngImages,
    TiffTifImages,
    BmpImages,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dImplicitGeometryMimeTypeValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/core/2.0/ImplicitGeometry_mimeType.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "model/vrml" => Self::Vrml97,
            "application/x-3ds" => Self::N3dsMax,
            "application/dxf" => Self::AutocadDxfApplicationDxf,
            "application/x-autocad" => Self::AutocadDxfApplicationXAutocad,
            "application/x-dxf" => Self::AutocadDxfApplicationXDxf,
            "application/acad" => Self::AutocadDwg,
            "application/x-shockwave-flash" => Self::Shockwave3d,
            "model/x3d+xml" => Self::X3dModelX3dXml,
            "model/x3d+binary" => Self::X3dModelX3dBinary,
            "model/x3d+vrml" => Self::X3dModelX3dVrml,
            "image/gif" => Self::GifImages,
            "image/jpeg" => Self::JpegJpgImages,
            "image/png" => Self::PngImages,
            "image/tiff" => Self::TiffTifImages,
            "image/bmp" => Self::BmpImages,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Vrml97 => "model/vrml",
            Self::N3dsMax => "application/x-3ds",
            Self::AutocadDxfApplicationDxf => "application/dxf",
            Self::AutocadDxfApplicationXAutocad => "application/x-autocad",
            Self::AutocadDxfApplicationXDxf => "application/x-dxf",
            Self::AutocadDwg => "application/acad",
            Self::Shockwave3d => "application/x-shockwave-flash",
            Self::X3dModelX3dXml => "model/x3d+xml",
            Self::X3dModelX3dBinary => "model/x3d+binary",
            Self::X3dModelX3dVrml => "model/x3d+vrml",
            Self::GifImages => "image/gif",
            Self::JpegJpgImages => "image/jpeg",
            Self::PngImages => "image/png",
            Self::TiffTifImages => "image/tiff",
            Self::BmpImages => "image/bmp",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::values::MimeTypeValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("model/vrml", Sig3dImplicitGeometryMimeTypeValue::Vrml97),
            ("image/bmp", Sig3dImplicitGeometryMimeTypeValue::BmpImages),
        ] {
            assert_eq!(
                Sig3dImplicitGeometryMimeTypeValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dImplicitGeometryMimeTypeValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dImplicitGeometryMimeTypeValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = MimeTypeValue::from(Code::new("model/vrml"));
        assert_eq!(
            wrapped.interpret::<Sig3dImplicitGeometryMimeTypeValue>(),
            Some(Sig3dImplicitGeometryMimeTypeValue::Vrml97)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = MimeTypeValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "model/vrml",
        ));
        assert_eq!(
            wrapped.interpret::<Sig3dImplicitGeometryMimeTypeValue>(),
            None
        );
    }
}
