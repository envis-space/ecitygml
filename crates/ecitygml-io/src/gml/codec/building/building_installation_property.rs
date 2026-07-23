use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_installation, serialize_building_installation,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::BuildingInstallationProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_installation_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<BuildingInstallationProperty, Error> {
    let parsed: GmlBuildingInstallationProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::BuildingInstallation.into()) {
        object = Some(deserialize_building_installation(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(BuildingInstallationProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_building_installation_property(
    building_installation_property: &BuildingInstallationProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        building_installation_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        building_installation_property.ownership(),
    ));
    if let Some(object) = building_installation_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_building_installation(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        CityGmlElement::BuildingInstallationProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingInstallationProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
