use crate::Error;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::fmt;

/// The CityGML version of a document, as declared by its root element's namespace.
///
/// Only [`CitygmlVersion::V3_0`] is supported for actual deserialization today — see
/// [`detect_citygml_version`] for how this is determined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CitygmlVersion {
    V1_0,
    V2_0,
    V3_0,
}

impl CitygmlVersion {
    /// Matches a namespace URI against the known CityGML core namespaces of each version.
    fn from_namespace_uri(uri: &str) -> Option<Self> {
        match uri {
            "http://www.opengis.net/citygml/1.0" => Some(Self::V1_0),
            "http://www.opengis.net/citygml/2.0" => Some(Self::V2_0),
            "http://www.opengis.net/citygml/3.0" => Some(Self::V3_0),
            _ => None,
        }
    }
}

impl fmt::Display for CitygmlVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1_0 => write!(f, "1.0"),
            Self::V2_0 => write!(f, "2.0"),
            Self::V3_0 => write!(f, "3.0"),
        }
    }
}

/// Determines the CityGML version of `xml_document` by scanning only its root element's
/// namespace declarations (`xmlns="..."` / `xmlns:prefix="..."`) for a recognized CityGML core
/// namespace URI (`http://www.opengis.net/citygml/{1.0,2.0,3.0}`).
///
/// This stops after the root element's start tag, so it is cheap even for large documents —
/// it never scans the document body. Returns `Ok(None)` if the root declares no recognized
/// CityGML core namespace at all (e.g. malformed input, or a non-CityGML GML document).
pub fn detect_citygml_version(xml_document: &[u8]) -> Result<Option<CitygmlVersion>, Error> {
    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                for attr in e.attributes() {
                    let attr = attr.map_err(quick_xml::Error::from)?;
                    if !attr.key.as_ref().starts_with(b"xmlns") {
                        continue;
                    }
                    let value = attr.decode_and_unescape_value(reader.decoder())?;
                    if let Some(version) = CitygmlVersion::from_namespace_uri(&value) {
                        return Ok(Some(version));
                    }
                }
                return Ok(None);
            }
            Ok(Event::Eof) => return Ok(None),
            Err(e) => return Err(Error::from(e)),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_v3_0_default_namespace() {
        let xml = br#"<?xml version="1.0"?>
            <CityModel xmlns:bldg="http://www.opengis.net/citygml/building/3.0"
                       xmlns="http://www.opengis.net/citygml/3.0">
            </CityModel>"#;

        assert_eq!(
            detect_citygml_version(xml).unwrap(),
            Some(CitygmlVersion::V3_0)
        );
    }

    #[test]
    fn test_detects_v2_0_prefixed_namespace() {
        let xml = br#"<?xml version="1.0"?>
            <core:CityModel xmlns:core="http://www.opengis.net/citygml/2.0">
            </core:CityModel>"#;

        assert_eq!(
            detect_citygml_version(xml).unwrap(),
            Some(CitygmlVersion::V2_0)
        );
    }

    #[test]
    fn test_detects_v1_0() {
        let xml = br#"<?xml version="1.0"?>
            <CityModel xmlns="http://www.opengis.net/citygml/1.0">
            </CityModel>"#;

        assert_eq!(
            detect_citygml_version(xml).unwrap(),
            Some(CitygmlVersion::V1_0)
        );
    }

    #[test]
    fn test_ignores_non_core_citygml_namespaces() {
        // A module namespace like `citygml/construction/3.0` must not match the core
        // `citygml/3.0` namespace check.
        let xml = br#"<?xml version="1.0"?>
            <CityModel xmlns:con="http://www.opengis.net/citygml/construction/3.0"
                       xmlns="http://www.opengis.net/citygml/3.0">
            </CityModel>"#;

        assert_eq!(
            detect_citygml_version(xml).unwrap(),
            Some(CitygmlVersion::V3_0)
        );
    }

    #[test]
    fn test_unknown_when_no_citygml_namespace_declared() {
        let xml = br#"<?xml version="1.0"?>
            <root xmlns="http://example.com/not-citygml"></root>"#;

        assert_eq!(detect_citygml_version(xml).unwrap(), None);
    }

    #[test]
    fn test_unknown_for_garbage_input() {
        let xml = b"not xml at all";

        assert_eq!(detect_citygml_version(xml).unwrap(), None);
    }
}
