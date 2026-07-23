use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_constructive_element, serialize_building_constructive_element,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::BuildingConstructiveElementProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_constructive_element_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<BuildingConstructiveElementProperty, Error> {
    let parsed: GmlBuildingConstructiveElementProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::BuildingConstructiveElement.into()) {
        object = Some(deserialize_building_constructive_element(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(BuildingConstructiveElementProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_building_constructive_element_property(
    building_constructive_element_property: &BuildingConstructiveElementProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        building_constructive_element_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        building_constructive_element_property.ownership(),
    ));
    if let Some(object) = building_constructive_element_property.object() {
        parts.content.push(XmlNodeContent::Child(
            serialize_building_constructive_element(object, formatting)?,
        ));
    }
    Ok(XmlNode::new(
        CityGmlElement::BuildingConstructiveElementProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingConstructiveElementProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
