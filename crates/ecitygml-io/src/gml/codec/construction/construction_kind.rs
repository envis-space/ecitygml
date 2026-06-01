use crate::Error;
use crate::gml::codec::building::deserialize_building_kind;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::construction::ConstructionKind;

pub fn deserialize_construction_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ConstructionKind>, Error> {
    let building_kind = deserialize_building_kind(xml_document, spans)?;
    if let Some(x) = building_kind {
        return Ok(Some(ConstructionKind::BuildingKind(x)));
    }

    Ok(None)
}
