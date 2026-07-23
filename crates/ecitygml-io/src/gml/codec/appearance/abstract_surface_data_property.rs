use crate::Error;
use crate::gml::codec::appearance::abstract_surface_data_kind::{
    deserialize_abstract_surface_data_kind, serialize_abstract_surface_data_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::appearance::AbstractSurfaceDataProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_surface_data_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractSurfaceDataProperty, Error> {
    let parsed: GmlSurfaceDataProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_surface_data_kind(xml_document, spans)?;

    Ok(AbstractSurfaceDataProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_surface_data_property(
    abstract_surface_data_property: &AbstractSurfaceDataProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();
    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            abstract_surface_data_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            abstract_surface_data_property.ownership(),
        ));

    if let Some(object) = abstract_surface_data_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_surface_data_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(
        CityGmlElement::AbstractSurfaceDataProperty.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurfaceDataProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
