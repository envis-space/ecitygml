use crate::Error;
use crate::gml::codec::transportation::{deserialize_intersection, serialize_intersection};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::transportation::IntersectionProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_intersection_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<IntersectionProperty, Error> {
    let parsed: GmlIntersectionProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::Intersection.into()) {
        object = Some(deserialize_intersection(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(IntersectionProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_intersection_property(
    intersection_property: &IntersectionProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        intersection_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        intersection_property.ownership(),
    ));
    if let Some(object) = intersection_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_intersection(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        CityGmlElement::IntersectionProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlIntersectionProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
