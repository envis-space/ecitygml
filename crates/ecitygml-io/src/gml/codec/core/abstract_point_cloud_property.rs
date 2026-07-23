use crate::Error;
use crate::gml::codec::core::abstract_point_cloud_kind::{
    deserialize_abstract_point_cloud_kind, serialize_abstract_point_cloud_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractPointCloudProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_point_cloud_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractPointCloudProperty, Error> {
    let parsed: GmlPointCloudProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_point_cloud_kind(xml_document, spans)?;

    Ok(AbstractPointCloudProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_point_cloud_property(
    abstract_point_cloud_property: &AbstractPointCloudProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_point_cloud_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_point_cloud_property.ownership(),
    ));

    if let Some(object) = abstract_point_cloud_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_point_cloud_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(
        CityGmlElement::AbstractPointCloudProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointCloudProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
