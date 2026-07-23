use crate::Error;
use crate::gml::codec::transportation::{
    deserialize_intersection, deserialize_railway, deserialize_road, deserialize_section,
    deserialize_square, deserialize_track, deserialize_waterway, serialize_intersection,
    serialize_railway, serialize_road, serialize_section, serialize_square, serialize_track,
    serialize_waterway,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::transportation::AbstractTransportationSpaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_transportation_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractTransportationSpaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::Section.into()) {
        let section = deserialize_section(&xml_document[span.start..span.end])?;
        return Ok(Some(section.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::Intersection.into()) {
        let intersection = deserialize_intersection(&xml_document[span.start..span.end])?;
        return Ok(Some(intersection.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::Road.into()) {
        let road = deserialize_road(&xml_document[span.start..span.end])?;
        return Ok(Some(road.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::Track.into()) {
        let track = deserialize_track(&xml_document[span.start..span.end])?;
        return Ok(Some(track.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::Railway.into()) {
        let railway = deserialize_railway(&xml_document[span.start..span.end])?;
        return Ok(Some(railway.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::Waterway.into()) {
        let waterway = deserialize_waterway(&xml_document[span.start..span.end])?;
        return Ok(Some(waterway.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::Square.into()) {
        let square = deserialize_square(&xml_document[span.start..span.end])?;
        return Ok(Some(square.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_transportation_space_kind(
    abstract_transportation_space_kind: &AbstractTransportationSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_transportation_space_kind {
        AbstractTransportationSpaceKind::Section(x) => serialize_section(x, formatting),
        AbstractTransportationSpaceKind::Intersection(x) => serialize_intersection(x, formatting),
        AbstractTransportationSpaceKind::Road(x) => serialize_road(x, formatting),
        AbstractTransportationSpaceKind::Track(x) => serialize_track(x, formatting),
        AbstractTransportationSpaceKind::Railway(x) => serialize_railway(x, formatting),
        AbstractTransportationSpaceKind::Waterway(x) => serialize_waterway(x, formatting),
        AbstractTransportationSpaceKind::Square(x) => serialize_square(x, formatting),
    }
}
