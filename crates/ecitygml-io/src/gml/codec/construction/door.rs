use crate::Error;
use crate::gml::codec::construction::abstract_filling_element::{
    deserialize_abstract_filling_element, serialize_abstract_filling_element,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractFillingElement, Door};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};

pub fn deserialize_door(xml_document: &[u8]) -> Result<Door, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_element = deserialize_abstract_filling_element(xml_document, &spans)?;
    let door = Door::from_abstract_filling_element(abstract_filling_element);

    Ok(door)
}

pub fn serialize_door(door: &Door, formatting: Formatting) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_filling_element(door.abstract_filling_element(), formatting)?;

    Ok(XmlNode::new(CityGmlElement::Door.into(), xml_node_parts))
}
