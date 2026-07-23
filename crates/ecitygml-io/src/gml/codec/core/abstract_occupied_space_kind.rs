use crate::Error;
use crate::gml::codec::city_furniture::{deserialize_city_furniture, serialize_city_furniture};
use crate::gml::codec::construction::{
    deserialize_abstract_construction_kind, deserialize_abstract_constructive_element_kind,
    deserialize_abstract_filling_element_kind, deserialize_abstract_installation_kind,
    serialize_abstract_construction_kind, serialize_abstract_constructive_element_kind,
    serialize_abstract_filling_element_kind, serialize_abstract_installation_kind,
};
use crate::gml::codec::generics::{
    deserialize_generic_occupied_space, serialize_generic_occupied_space,
};
use crate::gml::codec::vegetation::deserialize_abstract_vegetation_object_kind;
use crate::gml::codec::vegetation::serialize_abstract_vegetation_object_kind;
use crate::gml::codec::water_body::{deserialize_water_body, serialize_water_body};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractOccupiedSpaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_occupied_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractOccupiedSpaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::CityFurniture.into()) {
        let city_furniture = deserialize_city_furniture(&xml_document[span.start..span.end])?;
        return Ok(Some(city_furniture.into()));
    }
    if let Some(x) = deserialize_abstract_construction_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_abstract_constructive_element_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_abstract_filling_element_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::GenericOccupiedSpace.into()) {
        let generic_occupied_space =
            deserialize_generic_occupied_space(&xml_document[span.start..span.end])?;
        return Ok(Some(generic_occupied_space.into()));
    }
    if let Some(x) = deserialize_abstract_installation_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_abstract_vegetation_object_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::WaterBody.into()) {
        let water_body = deserialize_water_body(&xml_document[span.start..span.end])?;
        return Ok(Some(water_body.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_occupied_space_kind(
    abstract_occupied_space_kind: &AbstractOccupiedSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_occupied_space_kind {
        AbstractOccupiedSpaceKind::CityFurniture(x) => serialize_city_furniture(x, formatting),
        AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => {
            serialize_abstract_construction_kind(x, formatting)
        }
        AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
            serialize_abstract_constructive_element_kind(x, formatting)
        }
        AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => {
            serialize_abstract_filling_element_kind(x, formatting)
        }
        AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => {
            serialize_generic_occupied_space(x, formatting)
        }
        AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => {
            serialize_abstract_installation_kind(x, formatting)
        }
        AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => {
            serialize_abstract_vegetation_object_kind(x, formatting)
        }
        AbstractOccupiedSpaceKind::WaterBody(x) => serialize_water_body(x, formatting),
    }
}
