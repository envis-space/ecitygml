use crate::Error;
use crate::gml::parser::core::abstract_physical_space::deserialize_abstract_physical_space;
use ecitygml_core::model::core::AbstractUnoccupiedSpace;

pub fn deserialize_abstract_unoccupied_space(
    xml_document: &[u8],
) -> Result<AbstractUnoccupiedSpace, Error> {
    let abstract_physical_space = deserialize_abstract_physical_space(xml_document)?;
    let abstract_unoccupied_space = AbstractUnoccupiedSpace::new(abstract_physical_space);

    Ok(abstract_unoccupied_space)
}
