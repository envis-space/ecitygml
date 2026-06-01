use crate::Error;
use crate::gml::codec::core::occupied_space_kind::deserialize_occupied_space_kind;
use crate::gml::codec::core::unoccupied_space_kind::deserialize_unoccupied_space_kind;
use crate::gml::util::XmlElementSpans;
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
