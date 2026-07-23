use crate::Error;
use crate::gml::codec::core::city_object_relation::{
    deserialize_city_object_relation, serialize_city_object_relation,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::CityObjectRelationProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_city_object_relation_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<CityObjectRelationProperty, Error> {
    let parsed: GmlCityObjectRelationProperty = de::from_reader(xml_document)?;

    let mut object = None;
    if let Some(span) = spans.first(CityGmlElement::CityObjectRelation.into()) {
        object = Some(deserialize_city_object_relation(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(CityObjectRelationProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_city_object_relation_property(
    city_object_relation_property: &CityObjectRelationProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        city_object_relation_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        city_object_relation_property.ownership(),
    ));

    if let Some(object) = city_object_relation_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_city_object_relation(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(
        CityGmlElement::RelatedToProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCityObjectRelationProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
