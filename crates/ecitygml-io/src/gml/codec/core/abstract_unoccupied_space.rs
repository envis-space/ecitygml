use crate::Error;
use crate::gml::codec::core::abstract_physical_space::deserialize_abstract_physical_space;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::AbstractUnoccupiedSpace;

pub fn deserialize_abstract_unoccupied_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractUnoccupiedSpace, Error> {
    let abstract_physical_space = deserialize_abstract_physical_space(xml_document, spans)?;
    let abstract_unoccupied_space = AbstractUnoccupiedSpace::new(abstract_physical_space);

    Ok(abstract_unoccupied_space)
}
