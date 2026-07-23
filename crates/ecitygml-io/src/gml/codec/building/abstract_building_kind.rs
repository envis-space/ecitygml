use crate::Error;
use crate::gml::codec::building::{
    deserialize_building, deserialize_building_part, serialize_building, serialize_building_part,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::AbstractBuildingKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_building_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractBuildingKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::Building.into()) {
        let building = deserialize_building(&xml_document[span.start..span.end])?;
        return Ok(Some(building.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::BuildingPart.into()) {
        let building_part = deserialize_building_part(&xml_document[span.start..span.end])?;
        return Ok(Some(building_part.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_building_kind(
    abstract_building_kind: &AbstractBuildingKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_building_kind {
        AbstractBuildingKind::Building(x) => serialize_building(x, formatting),
        AbstractBuildingKind::BuildingPart(x) => serialize_building_part(x, formatting),
    }
}
