use crate::Error;
use crate::gml::codec::building::{deserialize_building_kind, serialize_building_kind};
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
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

pub fn serialize_construction_kind(
    construction_kind: &ConstructionKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match construction_kind {
        ConstructionKind::BuildingKind(x) => serialize_building_kind(x, formatting),
    }
}
