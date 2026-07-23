use crate::Error;
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::relief::TinProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::codec::geometry::primitives::{
    deserialize_triangulated_surface, serialize_triangulated_surface,
};
use egml::io::util::{
    Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts,
};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_tin_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<TinProperty, Error> {
    let parsed: GmlTinProperty = de::from_reader(xml_document)?;

    let mut object = None;
    if let Some(span) = spans.first(GmlElement::TriangulatedSurface.into()) {
        let ts_slice = &xml_document[span.start..span.end];
        object = Some(deserialize_triangulated_surface(ts_slice)?);
    };

    Ok(TinProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_tin_property(
    tin_property: &TinProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts
        .attributes
        .extend(serialize_association_attributes(tin_property.association()));
    parts
        .attributes
        .extend(serialize_ownership_attributes(tin_property.ownership()));

    if let Some(object) = tin_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_triangulated_surface(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(CityGmlElement::TinProperty.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTinProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
