use crate::Error;
use crate::gml::codec::core::appearance_kind::{
    deserialize_appearance_kind, serialize_appearance_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AppearanceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_appearance_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AppearanceProperty, Error> {
    let gml_appearance_property: GmlAppearanceProperty = de::from_reader(xml_document)?;
    let mut appearance_property: AppearanceProperty = gml_appearance_property.into();

    appearance_property.object = deserialize_appearance_kind(xml_document, spans)?;

    Ok(appearance_property)
}

pub fn serialize_appearance_property(
    appearance_property: &AppearanceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    if let Some(href) = &appearance_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &appearance_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_appearance_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(XmlElement::AppearanceProperty, parts))
}

pub fn serialize_appearance_member_property(
    appearance_property: &AppearanceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    if let Some(href) = &appearance_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &appearance_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_appearance_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(XmlElement::AppearanceMemberProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAppearanceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlAppearanceProperty> for AppearanceProperty {
    fn from(item: GmlAppearanceProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
