use crate::Error;
use crate::gml::codec::appearance::abstract_texture::{
    deserialize_abstract_texture, serialize_abstract_texture,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::appearance::{AsAbstractTexture, GeoreferencedTexture};
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_georeferenced_texture(
    xml_document: &[u8],
) -> Result<GeoreferencedTexture, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_texture_result, parsed_result) = rayon::join(
        || deserialize_abstract_texture(xml_document, &spans),
        || de::from_reader::<_, GmlGeoreferencedTexture>(xml_document).map_err(Error::from),
    );
    let abstract_texture = abstract_texture_result?;
    let parsed = parsed_result?;

    let mut georeferenced_texture = GeoreferencedTexture::from_abstract_texture(abstract_texture);
    georeferenced_texture.set_prefer_world_file(parsed.prefer_world_file);

    Ok(georeferenced_texture)
}

pub fn serialize_georeferenced_texture(
    georeferenced_texture: &GeoreferencedTexture,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_texture(georeferenced_texture.abstract_texture(), formatting)?;

    if let Some(raw) = serialize_inner(
        GmlGeoreferencedTexture::from(georeferenced_texture),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::GeoreferencedTexture.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlGeoreferencedTexture {
    #[serde(
        rename(serialize = "app:preferWorldFile", deserialize = "preferWorldFile"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefer_world_file: Option<bool>,
}

impl From<&GeoreferencedTexture> for GmlGeoreferencedTexture {
    fn from(item: &GeoreferencedTexture) -> Self {
        Self {
            prefer_world_file: item.prefer_world_file(),
        }
    }
}
