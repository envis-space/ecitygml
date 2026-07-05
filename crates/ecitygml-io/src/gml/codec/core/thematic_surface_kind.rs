use crate::Error;
use crate::gml::codec::construction::{
    deserialize_construction_surface_kind, deserialize_filling_surface_kind,
    serialize_construction_surface_kind, serialize_filling_surface_kind,
};
use crate::gml::codec::core::closure_surface::{
    deserialize_closure_surface, serialize_closure_surface,
};
use crate::gml::codec::generics::{
    deserialize_generic_thematic_surface, serialize_generic_thematic_surface,
};
use crate::gml::codec::land_use::{deserialize_land_use, serialize_land_use};
use crate::gml::codec::transportation::{
    deserialize_auxiliary_traffic_area, deserialize_marking, deserialize_traffic_area,
    serialize_auxiliary_traffic_area, serialize_marking, serialize_traffic_area,
};
use crate::gml::codec::water_body::{
    deserialize_water_boundary_surface_kind, serialize_water_boundary_surface_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::ThematicSurfaceKind;

pub fn deserialize_thematic_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ThematicSurfaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::AuxiliaryTrafficArea) {
        let auxiliary_traffic_area =
            deserialize_auxiliary_traffic_area(&xml_document[span.start..span.end])?;
        return Ok(Some(auxiliary_traffic_area.into()));
    }
    if let Some(span) = spans.first(XmlElement::ClosureSurface) {
        let closure_surface = deserialize_closure_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(closure_surface.into()));
    }
    if let Some(x) = deserialize_construction_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(x) = deserialize_filling_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(XmlElement::GenericThematicSurface) {
        let generic_thematic_surface =
            deserialize_generic_thematic_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(generic_thematic_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::LandUse) {
        let land_use = deserialize_land_use(&xml_document[span.start..span.end])?;
        return Ok(Some(land_use.into()));
    }
    if let Some(span) = spans.first(XmlElement::Marking) {
        let marking = deserialize_marking(&xml_document[span.start..span.end])?;
        return Ok(Some(marking.into()));
    }
    if let Some(span) = spans.first(XmlElement::TrafficArea) {
        let traffic_area = deserialize_traffic_area(&xml_document[span.start..span.end])?;
        return Ok(Some(traffic_area.into()));
    }
    if let Some(x) = deserialize_water_boundary_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_thematic_surface_kind(
    thematic_surface_kind: &ThematicSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match thematic_surface_kind {
        ThematicSurfaceKind::AuxiliaryTrafficArea(x) => {
            serialize_auxiliary_traffic_area(x, formatting)
        }
        ThematicSurfaceKind::ClosureSurface(x) => serialize_closure_surface(x, formatting),
        ThematicSurfaceKind::ConstructionSurfaceKind(x) => {
            serialize_construction_surface_kind(x, formatting)
        }
        ThematicSurfaceKind::FillingSurfaceKind(x) => serialize_filling_surface_kind(x, formatting),
        ThematicSurfaceKind::GenericThematicSurface(x) => {
            serialize_generic_thematic_surface(x, formatting)
        }
        ThematicSurfaceKind::LandUse(x) => serialize_land_use(x, formatting),
        ThematicSurfaceKind::Marking(x) => serialize_marking(x, formatting),
        ThematicSurfaceKind::TrafficArea(x) => serialize_traffic_area(x, formatting),
        ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => {
            serialize_water_boundary_surface_kind(x, formatting)
        }
    }
}
