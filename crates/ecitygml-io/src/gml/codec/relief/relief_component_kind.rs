use crate::Error;
use crate::gml::codec::relief::deserialize_tin_relief;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::relief::ReliefComponentKind;

pub fn deserialize_relief_component_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ReliefComponentKind>, Error> {
    if let Some(span) = spans.first(XmlElement::TINRelief) {
        let tin_surface = deserialize_tin_relief(&xml_document[span.start..span.end])?;
        return Ok(Some(ReliefComponentKind::TinRelief(tin_surface)));
    }

    Ok(None)
}
