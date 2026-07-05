use crate::Error;
use crate::gml::codec::city_furniture::{deserialize_city_furniture, serialize_city_furniture};
use crate::gml::codec::construction::{
    deserialize_construction_kind, deserialize_constructive_element_kind,
    deserialize_filling_element_kind, deserialize_installation_kind, serialize_construction_kind,
    serialize_constructive_element_kind, serialize_filling_element_kind,
    serialize_installation_kind,
};
use crate::gml::codec::generics::{
    deserialize_generic_occupied_space, serialize_generic_occupied_space,
};
use crate::gml::codec::vegetation::deserialize_vegetation_object_kind;
use crate::gml::codec::vegetation::serialize_vegetation_object_kind;
use crate::gml::codec::water_body::{deserialize_water_body, serialize_water_body};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::OccupiedSpaceKind;

pub fn deserialize_occupied_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<OccupiedSpaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::CityFurniture) {
        let city_furniture = deserialize_city_furniture(&xml_document[span.start..span.end])?;
        return Ok(Some(city_furniture.into()));
    }
    if let Some(x) = deserialize_construction_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_constructive_element_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_filling_element_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(XmlElement::GenericOccupiedSpace) {
        let generic_occupied_space =
            deserialize_generic_occupied_space(&xml_document[span.start..span.end])?;
        return Ok(Some(generic_occupied_space.into()));
    }
    if let Some(x) = deserialize_installation_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_vegetation_object_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(XmlElement::WaterBody) {
        let water_body = deserialize_water_body(&xml_document[span.start..span.end])?;
        return Ok(Some(water_body.into()));
    }

    Ok(None)
}

pub fn serialize_occupied_space_kind(
    occupied_space_kind: &OccupiedSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match occupied_space_kind {
        OccupiedSpaceKind::CityFurniture(x) => serialize_city_furniture(x, formatting),
        OccupiedSpaceKind::ConstructionKind(x) => serialize_construction_kind(x, formatting),
        OccupiedSpaceKind::ConstructiveElementKind(x) => {
            serialize_constructive_element_kind(x, formatting)
        }
        OccupiedSpaceKind::FillingElementKind(x) => serialize_filling_element_kind(x, formatting),
        OccupiedSpaceKind::GenericOccupiedSpace(x) => {
            serialize_generic_occupied_space(x, formatting)
        }
        OccupiedSpaceKind::InstallationKind(x) => serialize_installation_kind(x, formatting),
        OccupiedSpaceKind::VegetationObjectKind(x) => {
            serialize_vegetation_object_kind(x, formatting)
        }
        OccupiedSpaceKind::WaterBody(x) => serialize_water_body(x, formatting),
    }
}
