use crate::Error;
use crate::gml::parser::construction::deserialize_abstract_constructive_element;
use ecitygml_core::model::building::BuildingConstructiveElement;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_constructive_element(
    xml_document: &[u8],
) -> Result<BuildingConstructiveElement, Error> {
    let abstract_constructive_element = deserialize_abstract_constructive_element(xml_document)?;
    let mut building_constructive_element =
        BuildingConstructiveElement::new(abstract_constructive_element);
    let parsed_result: GmlAbstractConstructiveElement = de::from_reader(xml_document)?;

    building_constructive_element.set_class(parsed_result.class.map(|x| x.into()));
    building_constructive_element.set_functions(
        parsed_result
            .functions
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    building_constructive_element
        .set_usages(parsed_result.usages.into_iter().map(|x| x.into()).collect());

    Ok(building_constructive_element)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractConstructiveElement {
    #[serde(rename = "class", default)]
    pub class: Option<GmlCode>,

    #[serde(rename = "function", default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usages: Vec<GmlCode>,
}
