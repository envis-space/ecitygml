use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_unit, deserialize_storey, serialize_building_unit, serialize_storey,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingSubdivisionKind;

pub fn deserialize_building_subdivision_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<BuildingSubdivisionKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Storey) {
        let storey = deserialize_storey(&xml_document[span.start..span.end])?;
        return Ok(Some(storey.into()));
    }

    if let Some(span) = spans.first(XmlElement::BuildingUnit) {
        let building_unit = deserialize_building_unit(&xml_document[span.start..span.end])?;
        return Ok(Some(building_unit.into()));
    }

    Ok(None)
}

pub fn serialize_building_subdivision_kind(
    building_subdivision_kind: &BuildingSubdivisionKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match building_subdivision_kind {
        BuildingSubdivisionKind::BuildingUnit(x) => serialize_building_unit(x, formatting),
        BuildingSubdivisionKind::Storey(x) => serialize_storey(x, formatting),
    }
}
