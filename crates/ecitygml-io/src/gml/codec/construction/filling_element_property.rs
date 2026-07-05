use crate::Error;
use crate::gml::codec::construction::{
    deserialize_filling_element_kind, serialize_filling_element_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::FillingElementProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_filling_element_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<FillingElementProperty, Error> {
    let gml_filling_element_property: GmlFillingElementProperty = de::from_reader(xml_document)?;
    let mut filling_element_property: FillingElementProperty = gml_filling_element_property.into();

    filling_element_property.object = deserialize_filling_element_kind(xml_document, spans)?;

    Ok(filling_element_property)
}

pub fn serialize_filling_element_property(
    filling_element_property: &FillingElementProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &filling_element_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &filling_element_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_filling_element_kind(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::FillingElementProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlFillingElementProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlFillingElementProperty> for FillingElementProperty {
    fn from(item: GmlFillingElementProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
