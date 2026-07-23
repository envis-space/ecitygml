use crate::Error;
use crate::gml::codec::core::abstract_logical_space_kind::{
    deserialize_abstract_logical_space_kind, serialize_abstract_logical_space_kind,
};
use crate::gml::codec::core::abstract_physical_space_kind::{
    deserialize_abstract_physical_space_kind, serialize_abstract_physical_space_kind,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::AbstractSpaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractSpaceKind>, Error> {
    if let Some(x) = deserialize_abstract_logical_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_physical_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_space_kind(
    abstract_space_kind: &AbstractSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_space_kind {
        AbstractSpaceKind::AbstractLogicalSpaceKind(x) => {
            serialize_abstract_logical_space_kind(x, formatting)
        }
        AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => {
            serialize_abstract_physical_space_kind(x, formatting)
        }
    }
}
