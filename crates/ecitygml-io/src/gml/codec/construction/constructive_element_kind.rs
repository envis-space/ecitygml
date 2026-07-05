use crate::Error;
use crate::gml::codec::building::deserialize_building_constructive_element;
use crate::gml::codec::construction::{
    serialize_ceiling_surface, serialize_floor_surface, serialize_ground_surface,
    serialize_interior_wall_surface, serialize_outer_ceiling_surface,
    serialize_outer_floor_surface, serialize_roof_surface, serialize_wall_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{ConstructionSurfaceKind, ConstructiveElementKind};

pub fn deserialize_constructive_element_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ConstructiveElementKind>, Error> {
    if let Some(span) = spans.first(XmlElement::BuildingConstructiveElement) {
        let building_constructive_element =
            deserialize_building_constructive_element(&xml_document[span.start..span.end])?;
        return Ok(Some(building_constructive_element.into()));
    }

    Ok(None)
}

pub fn serialize_construction_surface_kind(
    construction_surface_kind: &ConstructionSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match construction_surface_kind {
        ConstructionSurfaceKind::CeilingSurface(x) => serialize_ceiling_surface(x, formatting),
        ConstructionSurfaceKind::FloorSurface(x) => serialize_floor_surface(x, formatting),
        ConstructionSurfaceKind::GroundSurface(x) => serialize_ground_surface(x, formatting),
        ConstructionSurfaceKind::InteriorWallSurface(x) => {
            serialize_interior_wall_surface(x, formatting)
        }
        ConstructionSurfaceKind::OuterCeilingSurface(x) => {
            serialize_outer_ceiling_surface(x, formatting)
        }
        ConstructionSurfaceKind::OuterFloorSurface(x) => {
            serialize_outer_floor_surface(x, formatting)
        }
        ConstructionSurfaceKind::RoofSurface(x) => serialize_roof_surface(x, formatting),
        ConstructionSurfaceKind::WallSurface(x) => serialize_wall_surface(x, formatting),
    }
}
