use crate::Error;
use crate::gml::codec::building::serialize_building_constructive_element_property;
use crate::gml::codec::core::{
    deserialize_abstract_logical_space, serialize_abstract_logical_space,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::building::values::{
    BuildingSubdivisionClassValue, BuildingSubdivisionFunctionValue, BuildingSubdivisionUsageValue,
};
use ecitygml_core::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};
use ecitygml_core::model::core::AsAbstractLogicalSpace;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_building_subdivision(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractBuildingSubdivision, Error> {
    let abstract_logical_space = deserialize_abstract_logical_space(xml_document, spans)?;
    let parsed =
        de::from_reader::<_, GmlAbstractBuildingSubdivision>(xml_document).map_err(Error::from)?;
    let mut abstract_building_subdivision =
        AbstractBuildingSubdivision::from_abstract_logical_space(abstract_logical_space);

    abstract_building_subdivision.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(BuildingSubdivisionClassValue::from),
    );
    abstract_building_subdivision.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(BuildingSubdivisionFunctionValue::from)
            .collect(),
    );
    abstract_building_subdivision.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(BuildingSubdivisionUsageValue::from)
            .collect(),
    );

    Ok(abstract_building_subdivision)
}

pub fn serialize_abstract_building_subdivision(
    abstract_building_subdivision: &AbstractBuildingSubdivision,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_logical_space(
        abstract_building_subdivision.abstract_logical_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractBuildingSubdivision::from(abstract_building_subdivision),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for prop in abstract_building_subdivision.building_constructive_elements() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_building_constructive_element_property(prop, formatting)?,
        ));
    }

    Ok(xml_node_parts)
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
}

impl From<&AbstractBuildingSubdivision> for GmlAbstractBuildingSubdivision {
    fn from(item: &AbstractBuildingSubdivision) -> Self {
        Self {
            class: item
                .class()
                .map(BuildingSubdivisionClassValue::code)
                .map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(BuildingSubdivisionFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(BuildingSubdivisionUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}
