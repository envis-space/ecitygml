use crate::Error;
use crate::gml::parser::core::deserialize_abstract_occupied_space;
use ecitygml_core::model::construction::AbstractInstallation;

pub fn deserialize_abstract_installation(
    xml_document: &[u8],
) -> Result<AbstractInstallation, Error> {
    let abstract_occupied_space = deserialize_abstract_occupied_space(xml_document)?;
    let abstract_installation = AbstractInstallation::new(abstract_occupied_space);

    Ok(abstract_installation)
}
