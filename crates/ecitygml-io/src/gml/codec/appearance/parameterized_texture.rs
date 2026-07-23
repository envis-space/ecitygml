use crate::Error;
use crate::gml::codec::appearance::abstract_texture::{
    deserialize_abstract_texture, serialize_abstract_texture,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::appearance::{AsAbstractTexture, ParameterizedTexture};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};

pub fn deserialize_parameterized_texture(
    xml_document: &[u8],
) -> Result<ParameterizedTexture, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_texture = deserialize_abstract_texture(xml_document, &spans)?;
    let parameterized_texture = ParameterizedTexture::from_abstract_texture(abstract_texture);

    Ok(parameterized_texture)
}

pub fn serialize_parameterized_texture(
    parameterized_texture: &ParameterizedTexture,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_texture(parameterized_texture.abstract_texture(), formatting)?;

    Ok(XmlNode::new(
        CityGmlElement::ParameterizedTexture.into(),
        xml_node_parts,
    ))
}
