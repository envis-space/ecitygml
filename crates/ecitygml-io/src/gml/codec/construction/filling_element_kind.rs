use crate::Error;
use crate::gml::codec::construction::door::deserialize_door;
use crate::gml::codec::construction::window::deserialize_window;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::construction::FillingElementKind;

pub fn deserialize_filling_element_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<FillingElementKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Door) {
        let door = deserialize_door(&xml_document[span.start..span.end])?;
        return Ok(Some(FillingElementKind::Door(door)));
    }
    if let Some(span) = spans.first(XmlElement::Window) {
        let window = deserialize_window(&xml_document[span.start..span.end])?;
        return Ok(Some(FillingElementKind::Window(window)));
    }

    Ok(None)
}
