use crate::Error;
use crate::gml::codec::transportation::deserialize_road;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::core::UnoccupiedSpaceKind;
use ecitygml_core::model::transportation::TransportationSpaceKind;

pub fn deserialize_unoccupied_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<UnoccupiedSpaceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Road) {
        let road = deserialize_road(&xml_document[span.start..span.end])?;
        return Ok(Some(UnoccupiedSpaceKind::TransportationSpaceKind(
            TransportationSpaceKind::Road(road),
        )));
    }

    Ok(None)
}
