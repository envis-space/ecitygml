use crate::Error;
use crate::gml::codec::construction::{
    deserialize_door_surface, deserialize_window_surface, serialize_door_surface,
    serialize_window_surface,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::AbstractFillingSurfaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_filling_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractFillingSurfaceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::DoorSurface.into()) {
        let door_surface = deserialize_door_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(door_surface.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::WindowSurface.into()) {
        let window_surface = deserialize_window_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(window_surface.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_filling_surface_kind(
    abstract_filling_surface_kind: &AbstractFillingSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_filling_surface_kind {
        AbstractFillingSurfaceKind::DoorSurface(x) => serialize_door_surface(x, formatting),
        AbstractFillingSurfaceKind::WindowSurface(x) => serialize_window_surface(x, formatting),
    }
}
