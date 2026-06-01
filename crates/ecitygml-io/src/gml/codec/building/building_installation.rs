use crate::Error;
use crate::gml::codec::construction::deserialize_abstract_installation;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::building::BuildingInstallation;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_installation(
    xml_document: &[u8],
) -> Result<BuildingInstallation, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_installation_result, parsed_result) = rayon::join(
        || deserialize_abstract_installation(xml_document, &spans),
        || de::from_reader::<_, GmlBuildingInstallation>(xml_document).map_err(Error::from),
    );
    let abstract_installation = abstract_installation_result?;
    let parsed = parsed_result?;
    let mut building_installation = BuildingInstallation::new(abstract_installation);

    building_installation.set_class(parsed.class.map(|x| x.into()));
    building_installation.set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    building_installation.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());

    Ok(building_installation)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingInstallation {
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
