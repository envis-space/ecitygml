use crate::Error;
use crate::gml::codec::city_furniture::deserialize_city_furniture;
use crate::gml::codec::construction::deserialize_construction_kind;
use crate::gml::codec::vegetation::deserialize_vegetation_object_kind;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::core::OccupiedSpaceKind;

pub fn deserialize_occupied_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<OccupiedSpaceKind>, Error> {
    if let Some(x) = deserialize_vegetation_object_kind(xml_document, spans)? {
        return Ok(Some(OccupiedSpaceKind::VegetationObjectKind(x)));
    }
    if let Some(x) = deserialize_construction_kind(xml_document, spans)? {
        return Ok(Some(OccupiedSpaceKind::ConstructionKind(x)));
    }

    if let Some(span) = spans.first(XmlElement::CityFurniture) {
        let city_furniture = deserialize_city_furniture(&xml_document[span.start..span.end])?;
        return Ok(Some(OccupiedSpaceKind::CityFurniture(city_furniture)));
    }

    Ok(None)
}
