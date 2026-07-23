use crate::Error;
use crate::gml::codec::building::serialize_building_constructive_element;
use crate::gml::codec::construction::{
    deserialize_ceiling_surface, deserialize_floor_surface, deserialize_ground_surface,
    deserialize_interior_wall_surface, deserialize_outer_ceiling_surface,
    deserialize_outer_floor_surface, deserialize_roof_surface, deserialize_wall_surface,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::{
    AbstractConstructionSurfaceKind, AbstractConstructiveElementKind,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_construction_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractConstructionSurfaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::CeilingSurface.into()) {
        let ceiling_surface = deserialize_ceiling_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ceiling_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::FloorSurface.into()) {
        let floor_surface = deserialize_floor_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(floor_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::GroundSurface.into()) {
        let ground_surface = deserialize_ground_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ground_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::InteriorWallSurface.into()) {
        let wall_surface = deserialize_interior_wall_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(wall_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::OuterCeilingSurface.into()) {
        let outer_ceiling_surface =
            deserialize_outer_ceiling_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(outer_ceiling_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::OuterFloorSurface.into()) {
        let outer_floor_surface =
            deserialize_outer_floor_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(outer_floor_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::RoofSurface.into()) {
        let roof_surface = deserialize_roof_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(roof_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::WallSurface.into()) {
        let wall_surface = deserialize_wall_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(wall_surface.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_constructive_element_kind(
    construction_element_kind: &AbstractConstructiveElementKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match construction_element_kind {
        AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
            serialize_building_constructive_element(x, formatting)
        }
    }
}
