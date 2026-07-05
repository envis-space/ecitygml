use crate::Error;
use crate::gml::codec::transportation::{deserialize_traffic_space, serialize_traffic_space};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::transportation::TrafficSpaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_traffic_space_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<TrafficSpaceProperty, Error> {
    let gml_traffic_space_property: GmlTrafficSpaceProperty = de::from_reader(xml_document)?;
    let mut traffic_space_property: TrafficSpaceProperty = gml_traffic_space_property.into();

    if let Some(span) = spans.first(XmlElement::TrafficSpace) {
        traffic_space_property.object = Some(deserialize_traffic_space(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(traffic_space_property)
}

pub fn serialize_traffic_space_property(
    traffic_space_property: &TrafficSpaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &traffic_space_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &traffic_space_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_traffic_space(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::TrafficSpaceProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficSpaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlTrafficSpaceProperty> for TrafficSpaceProperty {
    fn from(item: GmlTrafficSpaceProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
