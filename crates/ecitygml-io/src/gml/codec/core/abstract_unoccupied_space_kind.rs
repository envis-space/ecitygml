use crate::Error;
use crate::gml::codec::building::{deserialize_building_room, serialize_building_room};
use crate::gml::codec::transportation::{
    deserialize_abstract_transportation_space_kind, deserialize_auxiliary_traffic_space,
    deserialize_clearance_space, deserialize_hole, deserialize_traffic_space,
    serialize_abstract_transportation_space_kind, serialize_auxiliary_traffic_space,
    serialize_clearance_space, serialize_hole, serialize_traffic_space,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractUnoccupiedSpaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_unoccupied_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractUnoccupiedSpaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::AuxiliaryTrafficSpace.into()) {
        let auxiliary_traffic_space =
            deserialize_auxiliary_traffic_space(&xml_document[span.start..span.end])?;
        return Ok(Some(auxiliary_traffic_space.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::BuildingRoom.into()) {
        let building_room = deserialize_building_room(&xml_document[span.start..span.end])?;
        return Ok(Some(building_room.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::Hole.into()) {
        let hole = deserialize_hole(&xml_document[span.start..span.end])?;
        return Ok(Some(hole.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::ClearanceSpace.into()) {
        let clearance_space = deserialize_clearance_space(&xml_document[span.start..span.end])?;
        return Ok(Some(clearance_space.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::TrafficSpace.into()) {
        let traffic_space = deserialize_traffic_space(&xml_document[span.start..span.end])?;
        return Ok(Some(traffic_space.into()));
    }
    if let Some(x) = deserialize_abstract_transportation_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_unoccupied_space_kind(
    abstract_unoccupied_space_kind: &AbstractUnoccupiedSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_unoccupied_space_kind {
        AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => {
            serialize_auxiliary_traffic_space(x, formatting)
        }
        AbstractUnoccupiedSpaceKind::BuildingRoom(x) => serialize_building_room(x, formatting),
        AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => serialize_clearance_space(x, formatting),
        AbstractUnoccupiedSpaceKind::Hole(x) => serialize_hole(x, formatting),
        AbstractUnoccupiedSpaceKind::TrafficSpace(x) => serialize_traffic_space(x, formatting),
        AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
            serialize_abstract_transportation_space_kind(x, formatting)
        }
    }
}
