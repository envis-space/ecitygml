use crate::Error;
use crate::gml::codec::building::{
    deserialize_building, deserialize_building_part, serialize_building, serialize_building_part,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingKind;

pub fn deserialize_building_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<BuildingKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Building) {
        let building = deserialize_building(&xml_document[span.start..span.end])?;
        return Ok(Some(building.into()));
    }
    if let Some(span) = spans.first(XmlElement::BuildingPart) {
        let building_part = deserialize_building_part(&xml_document[span.start..span.end])?;
        return Ok(Some(building_part.into()));
    }

    Ok(None)
}

pub fn serialize_building_kind(
    building_kind: &BuildingKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match building_kind {
        BuildingKind::Building(x) => serialize_building(x, formatting),
        BuildingKind::BuildingPart(x) => serialize_building_part(x, formatting),
    }
}
