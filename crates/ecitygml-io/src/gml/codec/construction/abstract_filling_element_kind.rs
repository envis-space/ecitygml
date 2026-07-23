use crate::Error;
use crate::gml::codec::construction::door::{deserialize_door, serialize_door};
use crate::gml::codec::construction::window::{deserialize_window, serialize_window};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::AbstractFillingElementKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_filling_element_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractFillingElementKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::Door.into()) {
        let door = deserialize_door(&xml_document[span.start..span.end])?;
        return Ok(Some(door.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::Window.into()) {
        let window = deserialize_window(&xml_document[span.start..span.end])?;
        return Ok(Some(window.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_filling_element_kind(
    abstract_filling_element_kind: &AbstractFillingElementKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_filling_element_kind {
        AbstractFillingElementKind::Door(x) => serialize_door(x, formatting),
        AbstractFillingElementKind::Window(x) => serialize_window(x, formatting),
    }
}
