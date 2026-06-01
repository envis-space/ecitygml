use crate::Error;
use crate::gml::codec::building::abstract_building_subdivision::deserialize_abstract_building_subdivision;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::building::BuildingUnit;

pub fn deserialize_building_unit(xml_document: &[u8]) -> Result<BuildingUnit, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_building_subdivision =
        deserialize_abstract_building_subdivision(xml_document, &spans)?;
    let building_unit = BuildingUnit::new(abstract_building_subdivision);

    Ok(building_unit)
}
