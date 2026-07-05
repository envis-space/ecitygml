use crate::Error;
use crate::gml::codec::transportation::{deserialize_section, serialize_section};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::transportation::SectionProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_section_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<SectionProperty, Error> {
    let gml_section_property: GmlSectionProperty = de::from_reader(xml_document)?;
    let mut section_property: SectionProperty = gml_section_property.into();

    if let Some(span) = spans.first(XmlElement::Section) {
        section_property.object = Some(deserialize_section(&xml_document[span.start..span.end])?);
    }

    Ok(section_property)
}

pub fn serialize_section_property(
    section_property: &SectionProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &section_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &section_property.object {
        parts.content.push(XmlNodeContent::Child(serialize_section(
            object, formatting,
        )?));
    }
    Ok(XmlNode::new(XmlElement::SectionProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSectionProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlSectionProperty> for SectionProperty {
    fn from(item: GmlSectionProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
