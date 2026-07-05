use crate::Error;
use crate::gml::codec::transportation::{
    deserialize_intersection, deserialize_road, deserialize_section, serialize_intersection,
    serialize_road, serialize_section,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::transportation::TransportationSpaceKind;

pub fn deserialize_transportation_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<TransportationSpaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Section) {
        let section = deserialize_section(&xml_document[span.start..span.end])?;
        return Ok(Some(section.into()));
    }

    if let Some(span) = spans.first(XmlElement::Intersection) {
        let intersection = deserialize_intersection(&xml_document[span.start..span.end])?;
        return Ok(Some(intersection.into()));
    }

    if let Some(span) = spans.first(XmlElement::Road) {
        let road = deserialize_road(&xml_document[span.start..span.end])?;
        return Ok(Some(road.into()));
    }

    Ok(None)
}

pub fn serialize_transportation_space_kind(
    transportation_space_kind: &TransportationSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match transportation_space_kind {
        TransportationSpaceKind::Section(x) => serialize_section(x, formatting),
        TransportationSpaceKind::Intersection(x) => serialize_intersection(x, formatting),
        TransportationSpaceKind::Road(x) => serialize_road(x, formatting),
    }
}
