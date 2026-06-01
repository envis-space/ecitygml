use crate::Error;
use crate::gml::codec::construction::{deserialize_door_surface, deserialize_window_surface};
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::construction::FillingSurfaceKind;

pub fn deserialize_filling_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<FillingSurfaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::DoorSurface) {
        let door_surface = deserialize_door_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(FillingSurfaceKind::DoorSurface(door_surface)));
    }
    if let Some(span) = spans.first(XmlElement::WindowSurface) {
        let window_surface = deserialize_window_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(FillingSurfaceKind::WindowSurface(window_surface)));
    }

    Ok(None)
}
