use crate::Error;
use crate::gml::codec::transportation::deserialize_section;
use crate::gml::util::{XmlElement, XmlElementSpans};
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
