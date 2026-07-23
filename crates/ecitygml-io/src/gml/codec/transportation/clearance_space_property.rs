use crate::Error;
use crate::gml::codec::transportation::clearance_space::{
    deserialize_clearance_space, serialize_clearance_space,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::transportation::ClearanceSpaceProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_clearance_space_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<ClearanceSpaceProperty, Error> {
    let parsed: GmlClearanceSpaceProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::ClearanceSpace.into()) {
        object = Some(deserialize_clearance_space(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(ClearanceSpaceProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_clearance_space_property(
    clearance_space_property: &ClearanceSpaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        clearance_space_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        clearance_space_property.ownership(),
    ));
    if let Some(object) = clearance_space_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_clearance_space(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        CityGmlElement::ClearanceSpaceProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlClearanceSpaceProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
