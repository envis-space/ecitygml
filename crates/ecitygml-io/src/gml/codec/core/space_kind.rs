use crate::Error;
use crate::gml::codec::core::logical_space_kind::{
    deserialize_logical_space_kind, serialize_logical_space_kind,
};
use crate::gml::codec::core::physical_space_kind::{
    deserialize_physical_space_kind, serialize_physical_space_kind,
};
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::SpaceKind;

pub fn deserialize_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<SpaceKind>, Error> {
    if let Some(x) = deserialize_logical_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_physical_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_space_kind(
    space_kind: &SpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match space_kind {
        SpaceKind::LogicalSpaceKind(x) => serialize_logical_space_kind(x, formatting),
        SpaceKind::PhysicalSpaceKind(x) => serialize_physical_space_kind(x, formatting),
    }
}
