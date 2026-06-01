use crate::Error;
use crate::gml::codec::building::deserialize_storey;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::building::BuildingSubdivisionKind;

pub fn deserialize_building_subdivision_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<BuildingSubdivisionKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Storey) {
        let ground_surface = deserialize_storey(&xml_document[span.start..span.end])?;
        return Ok(Some(BuildingSubdivisionKind::Storey(ground_surface)));
    }

    Ok(None)
}
