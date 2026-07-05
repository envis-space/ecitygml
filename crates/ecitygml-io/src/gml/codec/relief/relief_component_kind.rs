use crate::Error;
use crate::gml::codec::relief::{deserialize_tin_relief, serialize_tin_relief};
use crate::gml::util::XmlElementSpans;
use crate::gml::util::XmlNode;
use crate::gml::util::xml_element::XmlElement;
use crate::gml::write::Formatting;
use ecitygml_core::model::relief::ReliefComponentKind;

pub fn deserialize_relief_component_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ReliefComponentKind>, Error> {
    if let Some(span) = spans.first(XmlElement::TINRelief) {
        let tin_surface = deserialize_tin_relief(&xml_document[span.start..span.end])?;
        return Ok(Some(tin_surface.into()));
    }

    Ok(None)
}

pub fn serialize_relief_component_kind(
    relief_component_kind: &ReliefComponentKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match relief_component_kind {
        ReliefComponentKind::TinRelief(x) => serialize_tin_relief(x, formatting),
    }
}
