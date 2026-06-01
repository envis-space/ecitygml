use crate::Error;
use crate::gml::codec::core::deserialize_abstract_occupied_space;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::construction::AbstractFillingElement;

pub fn deserialize_abstract_filling_element(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractFillingElement, Error> {
    let abstract_occupied_space = deserialize_abstract_occupied_space(xml_document, spans)?;
    let abstract_filling_element = AbstractFillingElement::new(abstract_occupied_space);

    Ok(abstract_filling_element)
}
