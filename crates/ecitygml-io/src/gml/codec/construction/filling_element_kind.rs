use crate::Error;
use crate::gml::codec::construction::door::{deserialize_door, serialize_door};
use crate::gml::codec::construction::window::{deserialize_window, serialize_window};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::FillingElementKind;

pub fn deserialize_filling_element_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<FillingElementKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Door) {
        let door = deserialize_door(&xml_document[span.start..span.end])?;
        return Ok(Some(door.into()));
    }
    if let Some(span) = spans.first(XmlElement::Window) {
        let window = deserialize_window(&xml_document[span.start..span.end])?;
        return Ok(Some(window.into()));
    }

    Ok(None)
}

pub fn serialize_filling_element_kind(
    filling_element_kind: &FillingElementKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match filling_element_kind {
        FillingElementKind::Door(x) => serialize_door(x, formatting),
        FillingElementKind::Window(x) => serialize_window(x, formatting),
    }
}
