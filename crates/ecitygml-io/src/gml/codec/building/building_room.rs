use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_unoccupied_space, serialize_abstract_unoccupied_space,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingRoom;
use ecitygml_core::model::core::AsAbstractUnoccupiedSpace;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_room(xml_document: &[u8]) -> Result<BuildingRoom, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_unoccupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_unoccupied_space(xml_document, &spans),
        || de::from_reader::<_, GmlAbstractBuildingRoom>(xml_document).map_err(Error::from),
    );
    let abstract_unoccupied_space = abstract_unoccupied_space_result?;
    let parsed = parsed_result?;
    let mut building_room = BuildingRoom::from_abstract_unoccupied_space(abstract_unoccupied_space);

    building_room.set_class(parsed.class.map(Into::into));
    building_room.set_functions(parsed.functions.into_iter().map(Into::into).collect());
    building_room.set_usages(parsed.usages.into_iter().map(Into::into).collect());

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

    Ok(XmlNode::new(XmlElement::BuildingRoom, xml_node_parts))
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
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
        }
    }
}
