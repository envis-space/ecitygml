use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_installation, serialize_building_installation,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingInstallationProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_installation_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<BuildingInstallationProperty, Error> {
    let gml_building_installation_property: GmlBuildingInstallationProperty =
        de::from_reader(xml_document)?;
    let mut building_installation_property: BuildingInstallationProperty =
        gml_building_installation_property.into();

    if let Some(span) = spans.first(XmlElement::BuildingInstallation) {
        building_installation_property.object = Some(deserialize_building_installation(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(building_installation_property)
}

pub fn serialize_building_installation_property(
    building_installation_property: &BuildingInstallationProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &building_installation_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &building_installation_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_building_installation(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        XmlElement::BuildingInstallationProperty,
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingInstallationProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlBuildingInstallationProperty> for BuildingInstallationProperty {
    fn from(item: GmlBuildingInstallationProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
