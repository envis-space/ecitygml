use crate::Error;
use crate::gml::codec::building::{deserialize_building_room, serialize_building_room};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingRoomProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_room_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<BuildingRoomProperty, Error> {
    let gml_building_room_property: GmlBuildingRoomProperty = de::from_reader(xml_document)?;
    let mut building_room_property: BuildingRoomProperty = gml_building_room_property.into();

    if let Some(span) = spans.first(XmlElement::BuildingRoom) {
        building_room_property.object = Some(deserialize_building_room(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(building_room_property)
}

pub fn serialize_building_room_property(
    building_room_property: &BuildingRoomProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &building_room_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &building_room_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_building_room(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::BuildingRoomProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingRoomProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlBuildingRoomProperty> for BuildingRoomProperty {
    fn from(item: GmlBuildingRoomProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
