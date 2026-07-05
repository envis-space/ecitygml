use crate::Error;
use crate::gml::codec::water_body::{
    deserialize_water_ground_surface, deserialize_water_surface, serialize_water_ground_surface,
    serialize_water_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::water_body::WaterBoundarySurfaceKind;

pub fn deserialize_water_boundary_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<WaterBoundarySurfaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::WaterGroundSurface) {
        let water_ground_surface =
            deserialize_water_ground_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(water_ground_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::WaterSurface) {
        let water_surface = deserialize_water_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(water_surface.into()));
    }

    Ok(None)
}

pub fn serialize_water_boundary_surface_kind(
    water_boundary_surface_kind: &WaterBoundarySurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match water_boundary_surface_kind {
        WaterBoundarySurfaceKind::WaterGroundSurface(x) => {
            serialize_water_ground_surface(x, formatting)
        }
        WaterBoundarySurfaceKind::WaterSurface(x) => serialize_water_surface(x, formatting),
    }
}
