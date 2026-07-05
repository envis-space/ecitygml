use crate::Error;
use crate::gml::codec::building::serialize_building_constructive_element;
use crate::gml::codec::construction::{
    deserialize_ceiling_surface, deserialize_floor_surface, deserialize_ground_surface,
    deserialize_interior_wall_surface, deserialize_outer_ceiling_surface,
    deserialize_outer_floor_surface, deserialize_roof_surface, deserialize_wall_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{ConstructionSurfaceKind, ConstructiveElementKind};

pub fn deserialize_construction_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ConstructionSurfaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::CeilingSurface) {
        let ceiling_surface = deserialize_ceiling_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ceiling_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::FloorSurface) {
        let floor_surface = deserialize_floor_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(floor_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::GroundSurface) {
        let ground_surface = deserialize_ground_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ground_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::InteriorWallSurface) {
        let wall_surface = deserialize_interior_wall_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(wall_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::OuterCeilingSurface) {
        let outer_ceiling_surface =
            deserialize_outer_ceiling_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(outer_ceiling_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::OuterFloorSurface) {
        let outer_floor_surface =
            deserialize_outer_floor_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(outer_floor_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::RoofSurface) {
        let roof_surface = deserialize_roof_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(roof_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::WallSurface) {
        let wall_surface = deserialize_wall_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(wall_surface.into()));
    }

    Ok(None)
}

pub fn serialize_constructive_element_kind(
    construction_element_kind: &ConstructiveElementKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match construction_element_kind {
        ConstructiveElementKind::BuildingConstructiveElement(x) => {
            serialize_building_constructive_element(x, formatting)
        }
    }
}
