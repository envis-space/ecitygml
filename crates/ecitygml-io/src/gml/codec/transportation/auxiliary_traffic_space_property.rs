use crate::Error;
use crate::gml::codec::transportation::{
    deserialize_auxiliary_traffic_space, serialize_auxiliary_traffic_space,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::transportation::AuxiliaryTrafficSpaceProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_auxiliary_traffic_space_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AuxiliaryTrafficSpaceProperty, Error> {
    let parsed: GmlAuxiliaryTrafficSpaceProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::AuxiliaryTrafficSpace.into()) {
        object = Some(deserialize_auxiliary_traffic_space(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(AuxiliaryTrafficSpaceProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_auxiliary_traffic_space_property(
    property: &AuxiliaryTrafficSpaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts
        .attributes
        .extend(serialize_association_attributes(property.association()));
    parts
        .attributes
        .extend(serialize_ownership_attributes(property.ownership()));
    if let Some(object) = property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_auxiliary_traffic_space(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        CityGmlElement::AuxiliaryTrafficSpaceProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAuxiliaryTrafficSpaceProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
