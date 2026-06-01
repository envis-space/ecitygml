use crate::Error;
use crate::gml::codec::building::deserialize_building;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::building::BuildingKind;

pub fn deserialize_building_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<BuildingKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Building) {
        let building = deserialize_building(&xml_document[span.start..span.end])?;
        return Ok(Some(BuildingKind::Building(building)));
    }

    Ok(None)
}
