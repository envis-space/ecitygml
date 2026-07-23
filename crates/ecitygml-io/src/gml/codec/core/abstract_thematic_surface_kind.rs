use crate::Error;
use crate::gml::codec::construction::{
    deserialize_abstract_construction_surface_kind, deserialize_abstract_filling_surface_kind,
    serialize_abstract_construction_surface_kind, serialize_abstract_filling_surface_kind,
};
use crate::gml::codec::core::closure_surface::{
    deserialize_closure_surface, serialize_closure_surface,
};
use crate::gml::codec::generics::{
    deserialize_generic_thematic_surface, serialize_generic_thematic_surface,
};
use crate::gml::codec::land_use::{deserialize_land_use, serialize_land_use};
use crate::gml::codec::transportation::{
    deserialize_auxiliary_traffic_area, deserialize_hole_surface, deserialize_marking,
    deserialize_traffic_area, serialize_auxiliary_traffic_area, serialize_hole_surface,
    serialize_marking, serialize_traffic_area,
};
use crate::gml::codec::water_body::{
    deserialize_abstract_water_boundary_surface_kind,
    serialize_abstract_water_boundary_surface_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractThematicSurfaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_thematic_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractThematicSurfaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::AuxiliaryTrafficArea.into()) {
        let auxiliary_traffic_area =
            deserialize_auxiliary_traffic_area(&xml_document[span.start..span.end])?;
        return Ok(Some(auxiliary_traffic_area.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::ClosureSurface.into()) {
        let closure_surface = deserialize_closure_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(closure_surface.into()));
    }
    if let Some(x) = deserialize_abstract_construction_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_abstract_filling_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::GenericThematicSurface.into()) {
        let generic_thematic_surface =
            deserialize_generic_thematic_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(generic_thematic_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::HoleSurface.into()) {
        let hole_surface = deserialize_hole_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(hole_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::LandUse.into()) {
        let land_use = deserialize_land_use(&xml_document[span.start..span.end])?;
        return Ok(Some(land_use.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::Marking.into()) {
        let marking = deserialize_marking(&xml_document[span.start..span.end])?;
        return Ok(Some(marking.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::TrafficArea.into()) {
        let traffic_area = deserialize_traffic_area(&xml_document[span.start..span.end])?;
        return Ok(Some(traffic_area.into()));
    }
    if let Some(x) = deserialize_abstract_water_boundary_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_thematic_surface_kind(
    abstract_thematic_surface_kind: &AbstractThematicSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_thematic_surface_kind {
        AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => {
            serialize_auxiliary_traffic_area(x, formatting)
        }
        AbstractThematicSurfaceKind::ClosureSurface(x) => serialize_closure_surface(x, formatting),
        AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
            serialize_abstract_construction_surface_kind(x, formatting)
        }
        AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
            serialize_abstract_filling_surface_kind(x, formatting)
        }
        AbstractThematicSurfaceKind::GenericThematicSurface(x) => {
            serialize_generic_thematic_surface(x, formatting)
        }
        AbstractThematicSurfaceKind::HoleSurface(x) => serialize_hole_surface(x, formatting),
        AbstractThematicSurfaceKind::LandUse(x) => serialize_land_use(x, formatting),
        AbstractThematicSurfaceKind::Marking(x) => serialize_marking(x, formatting),
        AbstractThematicSurfaceKind::TrafficArea(x) => serialize_traffic_area(x, formatting),
        AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
            serialize_abstract_water_boundary_surface_kind(x, formatting)
        }
    }
}
