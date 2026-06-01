use crate::Error;
use crate::gml::codec::building::deserialize_building_room;
use crate::gml::util::{XmlElement, XmlElementSpans};
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
