use crate::Error;
use crate::gml::parser::building::abstract_building_subdivision::deserialize_abstract_building_subdivision;
use ecitygml_core::model::building::Storey;

pub fn deserialize_storey(xml_document: &[u8]) -> Result<Storey, Error> {
    let abstract_building_subdivision = deserialize_abstract_building_subdivision(xml_document)?;
    let storey = Storey::new(abstract_building_subdivision);

    Ok(storey)
}
