use crate::Error;
use crate::gml::codec::appearance::surface_data_kind::{
    deserialize_surface_data_kind, serialize_surface_data_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::appearance::SurfaceDataProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_surface_data_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<SurfaceDataProperty, Error> {
    let gml_surface_data_property: GmlSurfaceDataProperty = de::from_reader(xml_document)?;
    let mut surface_data_property: SurfaceDataProperty = gml_surface_data_property.into();

    surface_data_property.object = deserialize_surface_data_kind(xml_document, spans)?;

    Ok(surface_data_property)
}

pub fn serialize_surface_data_property(
    surface_data_property: &SurfaceDataProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();
    if let Some(href) = &surface_data_property.href {
        xml_node_parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &surface_data_property.object {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_surface_data_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(
        XmlElement::SurfaceDataProperty,
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurfaceDataProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlSurfaceDataProperty> for SurfaceDataProperty {
    fn from(item: GmlSurfaceDataProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
