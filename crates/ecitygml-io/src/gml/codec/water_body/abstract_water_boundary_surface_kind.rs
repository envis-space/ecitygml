use crate::Error;
use crate::gml::codec::water_body::{
    deserialize_water_ground_surface, deserialize_water_surface, serialize_water_ground_surface,
    serialize_water_surface,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::water_body::AbstractWaterBoundarySurfaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_water_boundary_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractWaterBoundarySurfaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::WaterGroundSurface.into()) {
        let water_ground_surface =
            deserialize_water_ground_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(water_ground_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::WaterSurface.into()) {
        let water_surface = deserialize_water_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(water_surface.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_water_boundary_surface_kind(
    abstract_water_boundary_surface_kind: &AbstractWaterBoundarySurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_water_boundary_surface_kind {
        AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => {
            serialize_water_ground_surface(x, formatting)
        }
        AbstractWaterBoundarySurfaceKind::WaterSurface(x) => serialize_water_surface(x, formatting),
    }
}
