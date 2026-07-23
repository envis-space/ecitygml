use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_unoccupied_space, serialize_abstract_unoccupied_space,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::building::BuildingRoom;
use ecitygml_core::model::building::values::{
    BuildingRoomClassValue, BuildingRoomFunctionValue, BuildingRoomUsageValue,
};
use ecitygml_core::model::core::AsAbstractUnoccupiedSpace;
use egml::io::codec::basic::GmlCode;
use egml::io::util::extract_xml_element_spans;
use egml::io::util::{Formatting, XmlNode, XmlNodeContent, serialize_inner};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_room(xml_document: &[u8]) -> Result<BuildingRoom, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_unoccupied_space = deserialize_abstract_unoccupied_space(xml_document, &spans)?;
    let parsed =
        de::from_reader::<_, GmlAbstractBuildingRoom>(xml_document).map_err(Error::from)?;
    let mut building_room = BuildingRoom::from_abstract_unoccupied_space(abstract_unoccupied_space);

    building_room.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(BuildingRoomClassValue::from),
    );
    building_room.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(BuildingRoomFunctionValue::from)
            .collect(),
    );
    building_room.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(BuildingRoomUsageValue::from)
            .collect(),
    );

    Ok(building_room)
}

pub fn serialize_building_room(
    building_room: &BuildingRoom,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_unoccupied_space(building_room.abstract_unoccupied_space(), formatting)?;

    if let Some(raw) = serialize_inner(GmlAbstractBuildingRoom::from(building_room), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::BuildingRoom.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractBuildingRoom {
    #[serde(
        rename(serialize = "bldg:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "bldg:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "bldg:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&BuildingRoom> for GmlAbstractBuildingRoom {
    fn from(item: &BuildingRoom) -> Self {
        Self {
            class: item
                .class()
                .map(BuildingRoomClassValue::code)
                .map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(BuildingRoomFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(BuildingRoomUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}
