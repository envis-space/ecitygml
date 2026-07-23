use crate::Error;
use crate::gml::codec::transportation::{deserialize_section, serialize_section};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::transportation::SectionProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_section_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<SectionProperty, Error> {
    let parsed: GmlSectionProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::Section.into()) {
        object = Some(deserialize_section(&xml_document[span.start..span.end])?);
    }

    Ok(SectionProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_section_property(
    section_property: &SectionProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        section_property.association(),
    ));
    parts
        .attributes
        .extend(serialize_ownership_attributes(section_property.ownership()));
    if let Some(object) = section_property.object() {
        parts.content.push(XmlNodeContent::Child(serialize_section(
            object, formatting,
        )?));
    }
    Ok(XmlNode::new(CityGmlElement::SectionProperty.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSectionProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
