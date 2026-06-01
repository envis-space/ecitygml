use crate::Error;
use crate::gml::codec::building::deserialize_building_constructive_element;
use crate::gml::util::{XmlElement, XmlElementSpans};
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
