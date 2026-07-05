use crate::Error;
use crate::gml::codec::transportation::clearance_space::{
    deserialize_clearance_space, serialize_clearance_space,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::transportation::ClearanceSpaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_clearance_space_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<ClearanceSpaceProperty, Error> {
    let gml_clearance_space_property: GmlClearanceSpaceProperty = de::from_reader(xml_document)?;
    let mut clearance_space_property: ClearanceSpaceProperty = gml_clearance_space_property.into();

    if let Some(span) = spans.first(XmlElement::ClearanceSpace) {
        clearance_space_property.object = Some(deserialize_clearance_space(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(clearance_space_property)
}

pub fn serialize_clearance_space_property(
    clearance_space_property: &ClearanceSpaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &clearance_space_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &clearance_space_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_clearance_space(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::ClearanceSpaceProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlClearanceSpaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlClearanceSpaceProperty> for ClearanceSpaceProperty {
    fn from(item: GmlClearanceSpaceProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
