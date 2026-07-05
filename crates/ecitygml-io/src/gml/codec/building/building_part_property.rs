use crate::Error;
use crate::gml::codec::building::{deserialize_building_part, serialize_building_part};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingPartProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_part_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<BuildingPartProperty, Error> {
    let gml_building_part_property: GmlBuildingPartProperty = de::from_reader(xml_document)?;
    let mut building_part_property: BuildingPartProperty = gml_building_part_property.into();

    if let Some(span) = spans.first(XmlElement::BuildingPart) {
        building_part_property.object = Some(deserialize_building_part(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(building_part_property)
}

pub fn serialize_building_part_property(
    building_room_property: &BuildingPartProperty,
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
            .push(XmlNodeContent::Child(serialize_building_part(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::BuildingRoomProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingPartProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlBuildingPartProperty> for BuildingPartProperty {
    fn from(item: GmlBuildingPartProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
