use crate::Error;
use crate::gml::codec::construction::{
    deserialize_door_surface, deserialize_window_surface, serialize_door_surface,
    serialize_window_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::FillingSurfaceKind;

pub fn deserialize_filling_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<FillingSurfaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::DoorSurface) {
        let door_surface = deserialize_door_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(door_surface.into()));
    }
    if let Some(span) = spans.first(XmlElement::WindowSurface) {
        let window_surface = deserialize_window_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(window_surface.into()));
    }

    Ok(None)
}

pub fn serialize_filling_surface_kind(
    filling_surface_kind: &FillingSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match filling_surface_kind {
        FillingSurfaceKind::DoorSurface(x) => serialize_door_surface(x, formatting),
        FillingSurfaceKind::WindowSurface(x) => serialize_window_surface(x, formatting),
    }
}
