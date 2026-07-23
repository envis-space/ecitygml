use crate::Error;
use crate::gml::codec::building::deserialize_building_constructive_element;
use crate::gml::codec::construction::{
    serialize_ceiling_surface, serialize_floor_surface, serialize_ground_surface,
    serialize_interior_wall_surface, serialize_outer_ceiling_surface,
    serialize_outer_floor_surface, serialize_roof_surface, serialize_wall_surface,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::{
    AbstractConstructionSurfaceKind, AbstractConstructiveElementKind,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_constructive_element_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractConstructiveElementKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::BuildingConstructiveElement.into()) {
        let building_constructive_element =
            deserialize_building_constructive_element(&xml_document[span.start..span.end])?;
        return Ok(Some(building_constructive_element.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_construction_surface_kind(
    abstract_construction_surface_kind: &AbstractConstructionSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_construction_surface_kind {
        AbstractConstructionSurfaceKind::CeilingSurface(x) => {
            serialize_ceiling_surface(x, formatting)
        }
        AbstractConstructionSurfaceKind::FloorSurface(x) => serialize_floor_surface(x, formatting),
        AbstractConstructionSurfaceKind::GroundSurface(x) => {
            serialize_ground_surface(x, formatting)
        }
        AbstractConstructionSurfaceKind::InteriorWallSurface(x) => {
            serialize_interior_wall_surface(x, formatting)
        }
        AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => {
            serialize_outer_ceiling_surface(x, formatting)
        }
        AbstractConstructionSurfaceKind::OuterFloorSurface(x) => {
            serialize_outer_floor_surface(x, formatting)
        }
        AbstractConstructionSurfaceKind::RoofSurface(x) => serialize_roof_surface(x, formatting),
        AbstractConstructionSurfaceKind::WallSurface(x) => serialize_wall_surface(x, formatting),
    }
}
