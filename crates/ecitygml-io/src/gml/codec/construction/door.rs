use crate::Error;
use crate::gml::codec::construction::abstract_filling_element::{
    deserialize_abstract_filling_element, serialize_abstract_filling_element,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractFillingElement, Door};

pub fn deserialize_door(xml_document: &[u8]) -> Result<Door, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_element = deserialize_abstract_filling_element(xml_document, &spans)?;
    let door = Door::from_abstract_filling_element(abstract_filling_element);

    Ok(door)
}

pub fn serialize_door(door: &Door, formatting: Formatting) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_filling_element(door.abstract_filling_element(), formatting)?;

    Ok(XmlNode::new(XmlElement::Door, xml_node_parts))
}
