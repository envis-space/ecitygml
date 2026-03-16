use crate::Error;
use crate::gml::parser::building::abstract_building_subdivision::deserialize_abstract_building_subdivision;
use ecitygml_core::model::building::BuildingUnit;

pub fn deserialize_building_unit(xml_document: &[u8]) -> Result<BuildingUnit, Error> {
    let abstract_building_subdivision = deserialize_abstract_building_subdivision(xml_document)?;
    let building_unit = BuildingUnit::new(abstract_building_subdivision);

    Ok(building_unit)
}
