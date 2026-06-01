use crate::Error;
use crate::gml::codec::building::building_constructive_element_property::GmlBuildingConstructiveElementProperty;
use crate::gml::codec::core::deserialize_abstract_logical_space;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_building_subdivision(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractBuildingSubdivision, Error> {
    let (abstract_logical_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_logical_space(xml_document, spans),
        || de::from_reader::<_, GmlAbstractBuildingSubdivision>(xml_document).map_err(Error::from),
    );
    let abstract_logical_space = abstract_logical_space_result?;
    let parsed = parsed_result?;
    let mut abstract_building_subdivision =
        AbstractBuildingSubdivision::new(abstract_logical_space);

    abstract_building_subdivision.set_class(parsed.class.map(|x| x.into()));
    abstract_building_subdivision
        .set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    abstract_building_subdivision.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());

    Ok(abstract_building_subdivision)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractBuildingSubdivision {
    #[serde(
        rename(serialize = "bldg:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "bldg:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "bldg:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,

    #[serde(rename = "buildingConstructiveElement", default)]
    pub building_constructive_element: Vec<GmlBuildingConstructiveElementProperty>,
}
