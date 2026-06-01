use crate::Error;
use crate::gml::codec::construction::abstract_filling_element::deserialize_abstract_filling_element;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::construction::Door;

pub fn deserialize_door(xml_document: &[u8]) -> Result<Door, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_element = deserialize_abstract_filling_element(xml_document, &spans)?;
    let door = Door::new(abstract_filling_element);

    Ok(door)
}
