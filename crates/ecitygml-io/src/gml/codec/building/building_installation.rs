use crate::Error;
use crate::gml::codec::construction::{
    deserialize_abstract_installation, serialize_abstract_installation,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingInstallation;
use ecitygml_core::model::construction::AsAbstractInstallation;
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
    let mut building_installation =
        BuildingInstallation::from_abstract_installation(abstract_installation);

    building_installation.set_class(parsed.class.map(Into::into));
    building_installation.set_functions(parsed.functions.into_iter().map(Into::into).collect());
    building_installation.set_usages(parsed.usages.into_iter().map(Into::into).collect());

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
        XmlElement::BuildingInstallation,
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
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
        }
    }
}
