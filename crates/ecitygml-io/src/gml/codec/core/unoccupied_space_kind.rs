use crate::Error;
use crate::gml::codec::building::{deserialize_building_room, serialize_building_room};
use crate::gml::codec::transportation::{
    deserialize_auxiliary_traffic_space, deserialize_clearance_space, deserialize_traffic_space,
    deserialize_transportation_space_kind, serialize_auxiliary_traffic_space,
    serialize_clearance_space, serialize_traffic_space, serialize_transportation_space_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::UnoccupiedSpaceKind;

pub fn deserialize_unoccupied_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<UnoccupiedSpaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::BuildingRoom) {
        let building_room = deserialize_building_room(&xml_document[span.start..span.end])?;
        return Ok(Some(building_room.into()));
    }
    if let Some(span) = spans.first(XmlElement::ClearanceSpace) {
        let clearance_space = deserialize_clearance_space(&xml_document[span.start..span.end])?;
        return Ok(Some(clearance_space.into()));
    }
    if let Some(span) = spans.first(XmlElement::TrafficSpace) {
        let traffic_space = deserialize_traffic_space(&xml_document[span.start..span.end])?;
        return Ok(Some(traffic_space.into()));
    }
    if let Some(x) = deserialize_transportation_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(XmlElement::AuxiliaryTrafficSpace) {
        let auxiliary_traffic_space =
            deserialize_auxiliary_traffic_space(&xml_document[span.start..span.end])?;
        return Ok(Some(auxiliary_traffic_space.into()));
    }

    Ok(None)
}

pub fn serialize_unoccupied_space_kind(
    unoccupied_space_kind: &UnoccupiedSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match unoccupied_space_kind {
        UnoccupiedSpaceKind::BuildingRoom(x) => serialize_building_room(x, formatting),
        UnoccupiedSpaceKind::ClearanceSpace(x) => serialize_clearance_space(x, formatting),
        UnoccupiedSpaceKind::TrafficSpace(x) => serialize_traffic_space(x, formatting),
        UnoccupiedSpaceKind::TransportationSpaceKind(x) => {
            serialize_transportation_space_kind(x, formatting)
        }
        UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => {
            serialize_auxiliary_traffic_space(x, formatting)
        }
    }
}
