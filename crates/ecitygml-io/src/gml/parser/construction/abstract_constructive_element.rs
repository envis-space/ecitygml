use crate::Error;
use crate::gml::parser::core::deserialize_abstract_occupied_space;
use ecitygml_core::model::construction::AbstractConstructiveElement;

pub fn deserialize_abstract_constructive_element(
    xml_document: &[u8],
) -> Result<AbstractConstructiveElement, Error> {
    let abstract_occupied_space = deserialize_abstract_occupied_space(xml_document)?;
    let abstract_constructive_element = AbstractConstructiveElement::new(abstract_occupied_space);

    Ok(abstract_constructive_element)
}
