use crate::Error;
use crate::gml::parser::core::deserialize_abstract_unoccupied_space;
use ecitygml_core::model::building::BuildingRoom;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_room(xml_document: &[u8]) -> Result<BuildingRoom, Error> {
    let abstract_unoccupied_space = deserialize_abstract_unoccupied_space(xml_document)?;
    let mut building_room = BuildingRoom::new(abstract_unoccupied_space);
    let parsed_result: GmlAbstractBuildingRoom = de::from_reader(xml_document)?;

    building_room.set_class(parsed_result.class.map(|x| x.into()));
    building_room.set_functions(
        parsed_result
            .functions
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    building_room.set_usages(parsed_result.usages.into_iter().map(|x| x.into()).collect());

    Ok(building_room)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractBuildingRoom {
    #[serde(rename = "class", default)]
    pub class: Option<GmlCode>,

    #[serde(rename = "function", default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usages: Vec<GmlCode>,
}
