use crate::Error;
use crate::gml::codec::construction::{
    deserialize_abstract_installation, serialize_abstract_installation,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::building::BuildingInstallation;
use ecitygml_core::model::building::values::{
    BuildingInstallationClassValue, BuildingInstallationFunctionValue,
    BuildingInstallationUsageValue,
};
use ecitygml_core::model::construction::AsAbstractInstallation;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_installation(
    xml_document: &[u8],
) -> Result<BuildingInstallation, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_installation = deserialize_abstract_installation(xml_document, &spans)?;
    let parsed =
        de::from_reader::<_, GmlBuildingInstallation>(xml_document).map_err(Error::from)?;
    let mut building_installation =
        BuildingInstallation::from_abstract_installation(abstract_installation);

    building_installation.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(BuildingInstallationClassValue::from),
    );
    building_installation.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(BuildingInstallationFunctionValue::from)
            .collect(),
    );
    building_installation.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(BuildingInstallationUsageValue::from)
            .collect(),
    );

    Ok(building_installation)
}

pub fn serialize_building_installation(
    building_installation: &BuildingInstallation,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_installation(building_installation.abstract_installation(), formatting)?;

    if let Some(raw) = serialize_inner(
        GmlBuildingInstallation::from(building_installation),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::BuildingInstallation.into(),
        xml_node_parts,
    ))
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

impl From<&BuildingInstallation> for GmlBuildingInstallation {
    fn from(item: &BuildingInstallation) -> Self {
        Self {
            class: item
                .class()
                .map(BuildingInstallationClassValue::code)
                .map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(BuildingInstallationFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(BuildingInstallationUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}
