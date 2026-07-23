use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_unit, deserialize_storey, serialize_building_unit, serialize_storey,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::AbstractBuildingSubdivisionKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_building_subdivision_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractBuildingSubdivisionKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::Storey.into()) {
        let storey = deserialize_storey(&xml_document[span.start..span.end])?;
        return Ok(Some(storey.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::BuildingUnit.into()) {
        let building_unit = deserialize_building_unit(&xml_document[span.start..span.end])?;
        return Ok(Some(building_unit.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_building_subdivision_kind(
    abstract_building_subdivision_kind: &AbstractBuildingSubdivisionKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_building_subdivision_kind {
        AbstractBuildingSubdivisionKind::BuildingUnit(x) => serialize_building_unit(x, formatting),
        AbstractBuildingSubdivisionKind::Storey(x) => serialize_storey(x, formatting),
    }
}
