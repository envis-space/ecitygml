use crate::Error;
use crate::gml::parser::core::parse_abstract_occupied_space;
use ecitygml_core::model::building::BuildingConstructiveElement;

pub fn parse_building_constructive_element(
    xml_document: &[u8],
) -> Result<BuildingConstructiveElement, Error> {
    let abstract_occupied_space = parse_abstract_occupied_space(xml_document)?;
    let building_constructive_element = BuildingConstructiveElement::new(abstract_occupied_space);

    Ok(building_constructive_element)
}
