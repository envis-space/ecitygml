use crate::Error;
use crate::gml::codec::construction::{
    deserialize_ground_surface, deserialize_roof_surface, deserialize_wall_surface,
};
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::construction::ConstructionSurfaceKind;

pub fn deserialize_construction_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ConstructionSurfaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::GroundSurface) {
        let ground_surface = deserialize_ground_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ConstructionSurfaceKind::GroundSurface(ground_surface)));
    }
    if let Some(span) = spans.first(XmlElement::RoofSurface) {
        let roof_surface = deserialize_roof_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ConstructionSurfaceKind::RoofSurface(roof_surface)));
    }
    if let Some(span) = spans.first(XmlElement::WallSurface) {
        let wall_surface = deserialize_wall_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(ConstructionSurfaceKind::WallSurface(wall_surface)));
    }

    Ok(None)
}
