use crate::Error;
use crate::gml::codec::construction::{
    deserialize_abstract_filling_element_kind, serialize_abstract_filling_element_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::AbstractFillingElementProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_filling_element_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractFillingElementProperty, Error> {
    let parsed: GmlFillingElementProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_filling_element_kind(xml_document, spans)?;

    Ok(AbstractFillingElementProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_filling_element_property(
    abstract_filling_element_property: &AbstractFillingElementProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        abstract_filling_element_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_filling_element_property.ownership(),
    ));
    if let Some(object) = abstract_filling_element_property.object() {
        parts.content.push(XmlNodeContent::Child(
            serialize_abstract_filling_element_kind(object, formatting)?,
        ));
    }
    Ok(XmlNode::new(
        CityGmlElement::AbstractFillingElementProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlFillingElementProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
