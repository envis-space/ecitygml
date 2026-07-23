use crate::Error;
use crate::gml::codec::building::{deserialize_building_part, serialize_building_part};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::BuildingPartProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_part_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<BuildingPartProperty, Error> {
    let parsed: GmlBuildingPartProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::BuildingPart.into()) {
        object = Some(deserialize_building_part(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(BuildingPartProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_building_part_property(
    building_part_property: &BuildingPartProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        building_part_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        building_part_property.ownership(),
    ));
    if let Some(object) = building_part_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_building_part(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        CityGmlElement::BuildingPartProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingPartProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
