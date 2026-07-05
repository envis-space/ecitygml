use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AsAbstractOccupiedSpace;
use ecitygml_core::model::water_body::WaterBody;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_water_body(xml_document: &[u8]) -> Result<WaterBody, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_occupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_occupied_space(xml_document, &spans),
        || de::from_reader::<_, GmlWaterBody>(xml_document).map_err(Error::from),
    );
    let abstract_occupied_space = abstract_occupied_space_result?;
    let parsed = parsed_result?;
    let mut water_body = WaterBody::from_abstract_occupied_space(abstract_occupied_space);

    water_body.set_class(parsed.class.map(Into::into));
    water_body.set_functions(parsed.functions.into_iter().map(Into::into).collect());
    water_body.set_usages(parsed.usages.into_iter().map(Into::into).collect());

    Ok(water_body)
}

pub fn serialize_water_body(
    water_body: &WaterBody,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_occupied_space(water_body.abstract_occupied_space(), formatting)?;

    if let Some(raw) = serialize_inner(GmlWaterBody::from(water_body), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::WaterBody, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlWaterBody {
    #[serde(
        rename(serialize = "wtr:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "wtr:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "wtr:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&WaterBody> for GmlWaterBody {
    fn from(item: &WaterBody) -> Self {
        Self {
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
        }
    }
}
