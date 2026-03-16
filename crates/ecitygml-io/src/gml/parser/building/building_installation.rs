use crate::Error;
use crate::gml::parser::construction::deserialize_abstract_installation;
use ecitygml_core::model::building::BuildingInstallation;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_installation(
    xml_document: &[u8],
) -> Result<BuildingInstallation, Error> {
    let abstract_installation = deserialize_abstract_installation(xml_document)?;
    let mut building_installation = BuildingInstallation::new(abstract_installation);
    let parsed_result: GmlBuildingInstallation = de::from_reader(xml_document)?;

    building_installation.set_class(parsed_result.class.map(|x| x.into()));
    building_installation.set_functions(
        parsed_result
            .functions
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    building_installation.set_usages(parsed_result.usages.into_iter().map(|x| x.into()).collect());

    Ok(building_installation)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingInstallation {
    #[serde(rename = "class", default)]
    pub class: Option<GmlCode>,

    #[serde(rename = "function", default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usages: Vec<GmlCode>,
}
