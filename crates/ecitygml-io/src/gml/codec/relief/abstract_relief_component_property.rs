use crate::Error;
use crate::gml::codec::relief::abstract_relief_component_kind::{
    deserialize_abstract_relief_component_kind, serialize_abstract_relief_component_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::relief::AbstractReliefComponentProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_relief_component_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractReliefComponentProperty, Error> {
    let parsed: GmlReliefComponentProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_relief_component_kind(xml_document, spans)?;

    Ok(AbstractReliefComponentProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_relief_component_property(
    abstract_relief_component_property: &AbstractReliefComponentProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        abstract_relief_component_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_relief_component_property.ownership(),
    ));

    if let Some(object) = abstract_relief_component_property.object() {
        parts.content.push(XmlNodeContent::Child(
            serialize_abstract_relief_component_kind(object, formatting)?,
        ));
    }

    Ok(XmlNode::new(
        CityGmlElement::AbstractReliefComponentProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlReliefComponentProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
