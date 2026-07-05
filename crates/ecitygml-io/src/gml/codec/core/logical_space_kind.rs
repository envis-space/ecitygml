use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_subdivision_kind, serialize_building_subdivision_kind,
};
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::LogicalSpaceKind;

pub fn deserialize_logical_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<LogicalSpaceKind>, Error> {
    if let Some(x) = deserialize_building_subdivision_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_logical_space_kind(
    logical_space_kind: &LogicalSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match logical_space_kind {
        LogicalSpaceKind::BuildingSubdivisionKind(x) => {
            serialize_building_subdivision_kind(x, formatting)
        }
    }
}
