use crate::gml::codec::core::point_cloud_kind::{
    deserialize_point_cloud_kind, serialize_point_cloud_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::{Error, Formatting};
use ecitygml_core::model::core::PointCloudProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_point_cloud_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<PointCloudProperty, Error> {
    let gml_point_cloud_property: GmlPointCloudProperty = de::from_reader(xml_document)?;
    let mut point_cloud_property: PointCloudProperty = gml_point_cloud_property.into();

    point_cloud_property.object = deserialize_point_cloud_kind(xml_document, spans)?;

    Ok(point_cloud_property)
}

pub fn serialize_point_cloud_property(
    point_cloud_property: &PointCloudProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    if let Some(href) = &point_cloud_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &point_cloud_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_point_cloud_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(XmlElement::PointCloudProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointCloudProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlPointCloudProperty> for PointCloudProperty {
    fn from(item: GmlPointCloudProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
