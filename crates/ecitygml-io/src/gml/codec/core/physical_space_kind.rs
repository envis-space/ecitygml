use crate::Error;
use crate::gml::codec::core::occupied_space_kind::{
    deserialize_occupied_space_kind, serialize_occupied_space_kind,
};
use crate::gml::codec::core::unoccupied_space_kind::{
    deserialize_unoccupied_space_kind, serialize_unoccupied_space_kind,
};
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::PhysicalSpaceKind;

pub fn deserialize_physical_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<PhysicalSpaceKind>, Error> {
    if let Some(x) = deserialize_occupied_space_kind(xml_document, spans)? {
        return Ok(Some(PhysicalSpaceKind::OccupiedSpaceKind(x)));
    }

    if let Some(x) = deserialize_unoccupied_space_kind(xml_document, spans)? {
        return Ok(Some(PhysicalSpaceKind::UnoccupiedSpaceKind(x)));
    }

    Ok(None)
}

pub fn serialize_physical_space_kind(
    physical_space_kind: &PhysicalSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match physical_space_kind {
        PhysicalSpaceKind::OccupiedSpaceKind(x) => serialize_occupied_space_kind(x, formatting),
        PhysicalSpaceKind::UnoccupiedSpaceKind(x) => serialize_unoccupied_space_kind(x, formatting),
    }
}
