use crate::Error;
use crate::gml::codec::building::deserialize_building_installation;
use crate::gml::util::{XmlElement, XmlElementSpans};
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
