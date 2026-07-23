use crate::Error;
use crate::gml::codec::core::implicit_geometry::{
    deserialize_implicit_geometry, serialize_implicit_geometry,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::ImplicitGeometryProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_implicit_geometry_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<ImplicitGeometryProperty, Error> {
    let parsed: GmlImplicitGeometryProperty = de::from_reader(xml_document)?;

    let mut object = None;
    if let Some(span) = spans.first(CityGmlElement::ImplicitGeometry.into()) {
        object = Some(deserialize_implicit_geometry(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(ImplicitGeometryProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_implicit_geometry_property(
    implicit_geometry_property: &ImplicitGeometryProperty,
    formatting: Formatting,
    target_xml_element: CityGmlElement,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        implicit_geometry_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        implicit_geometry_property.ownership(),
    ));
    if let Some(object) = implicit_geometry_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_implicit_geometry(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlImplicitGeometryProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
