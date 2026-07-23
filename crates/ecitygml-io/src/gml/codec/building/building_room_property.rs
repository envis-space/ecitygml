use crate::Error;
use crate::gml::codec::building::{deserialize_building_room, serialize_building_room};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::BuildingRoomProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_room_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<BuildingRoomProperty, Error> {
    let parsed: GmlBuildingRoomProperty = de::from_reader(xml_document)?;

    let mut object = None;

    if let Some(span) = spans.first(CityGmlElement::BuildingRoom.into()) {
        object = Some(deserialize_building_room(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(BuildingRoomProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_building_room_property(
    building_room_property: &BuildingRoomProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        building_room_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        building_room_property.ownership(),
    ));
    if let Some(object) = building_room_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_building_room(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        CityGmlElement::BuildingRoomProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingRoomProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}
