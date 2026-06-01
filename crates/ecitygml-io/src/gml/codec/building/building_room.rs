use crate::Error;
use crate::gml::codec::core::deserialize_abstract_unoccupied_space;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::building::BuildingRoom;
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
    let mut building_room = BuildingRoom::new(abstract_unoccupied_space);

    building_room.set_class(parsed.class.map(|x| x.into()));
    building_room.set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    building_room.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());

    Ok(building_room)
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
