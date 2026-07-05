use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_constructive_element, serialize_building_constructive_element,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingConstructiveElementProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_constructive_element_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<BuildingConstructiveElementProperty, Error> {
    let gml_building_constructive_element_property: GmlBuildingConstructiveElementProperty =
        de::from_reader(xml_document)?;
    let mut building_constructive_element_property: BuildingConstructiveElementProperty =
        gml_building_constructive_element_property.into();

    if let Some(span) = spans.first(XmlElement::BuildingConstructiveElement) {
        building_constructive_element_property.object = Some(
            deserialize_building_constructive_element(&xml_document[span.start..span.end])?,
        );
    }

    Ok(building_constructive_element_property)
}

pub fn serialize_building_constructive_element_property(
    building_constructive_element_property: &BuildingConstructiveElementProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &building_constructive_element_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &building_constructive_element_property.object {
        parts.content.push(XmlNodeContent::Child(
            serialize_building_constructive_element(object, formatting)?,
        ));
    }
    Ok(XmlNode::new(
        XmlElement::BuildingConstructiveElementProperty,
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingConstructiveElementProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlBuildingConstructiveElementProperty> for BuildingConstructiveElementProperty {
    fn from(item: GmlBuildingConstructiveElementProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
