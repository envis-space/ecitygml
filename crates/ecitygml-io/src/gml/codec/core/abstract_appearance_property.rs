use crate::Error;
use crate::gml::codec::core::abstract_appearance_kind::{
    deserialize_abstract_appearance_kind, serialize_abstract_appearance_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractAppearanceProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_appearance_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractAppearanceProperty, Error> {
    let parsed: GmlAppearanceProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_appearance_kind(xml_document, spans)?;

    Ok(AbstractAppearanceProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_appearance_property(
    abstract_appearance_property: &AbstractAppearanceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_appearance_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_appearance_property.ownership(),
    ));

    if let Some(object) = abstract_appearance_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_appearance_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(
        CityGmlElement::AbstractAppearanceProperty.into(),
        parts,
    ))
}

pub fn serialize_abstract_appearance_member_property(
    abstract_appearance_property: &AbstractAppearanceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_appearance_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_appearance_property.ownership(),
    ));

    if let Some(object) = abstract_appearance_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_appearance_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(
        CityGmlElement::AppearanceMemberProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAppearanceProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
