use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use ecitygml_core::model::land_use::LandUse;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_land_use(xml_document: &[u8]) -> Result<LandUse, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_thematic_surface_result, parsed_result) = rayon::join(
        || deserialize_abstract_thematic_surface(xml_document, &spans),
        || de::from_reader::<_, GmlLandUse>(xml_document).map_err(Error::from),
    );
    let abstract_thematic_surface = abstract_thematic_surface_result?;
    let parsed = parsed_result?;

    let mut land_use = LandUse::from_abstract_thematic_surface(abstract_thematic_surface);
    land_use.set_class(parsed.class.map(Into::into));
    land_use.set_functions(parsed.functions.into_iter().map(Into::into).collect());
    land_use.set_usages(parsed.usages.into_iter().map(Into::into).collect());

    Ok(land_use)
}

pub fn serialize_land_use(land_use: &LandUse, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut parts =
        serialize_abstract_thematic_surface(land_use.abstract_thematic_surface(), formatting)?;

    if let Some(raw) = serialize_inner(GmlLandUse::from(land_use), formatting)? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::LandUse, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLandUse {
    #[serde(
        rename(serialize = "luse:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "luse:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "luse:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&LandUse> for GmlLandUse {
    fn from(item: &LandUse) -> Self {
        Self {
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
        }
    }
}
