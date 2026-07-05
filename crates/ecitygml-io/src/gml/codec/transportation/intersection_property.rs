use crate::Error;
use crate::gml::codec::transportation::{deserialize_intersection, serialize_intersection};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::transportation::IntersectionProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_intersection_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<IntersectionProperty, Error> {
    let gml_intersection_property: GmlIntersectionProperty = de::from_reader(xml_document)?;
    let mut intersection_property: IntersectionProperty = gml_intersection_property.into();

    if let Some(span) = spans.first(XmlElement::Intersection) {
        intersection_property.object = Some(deserialize_intersection(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(intersection_property)
}

pub fn serialize_intersection_property(
    intersection_property: &IntersectionProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &intersection_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &intersection_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_intersection(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::IntersectionProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlIntersectionProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlIntersectionProperty> for IntersectionProperty {
    fn from(item: GmlIntersectionProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
