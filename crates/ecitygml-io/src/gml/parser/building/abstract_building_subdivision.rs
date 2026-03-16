use crate::Error;
use crate::gml::parser::core::deserialize_abstract_logical_space;
use ecitygml_core::model::building::AbstractBuildingSubdivision;

pub fn deserialize_abstract_building_subdivision(
    xml_document: &[u8],
) -> Result<AbstractBuildingSubdivision, Error> {
    let abstract_logical_space = deserialize_abstract_logical_space(xml_document)?;
    let abstract_building_subdivision = AbstractBuildingSubdivision::new(abstract_logical_space);

    Ok(abstract_building_subdivision)
}
