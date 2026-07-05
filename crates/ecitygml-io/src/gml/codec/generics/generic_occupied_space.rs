use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::codec::vegetation::GmlPlantCover;
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AsAbstractOccupiedSpace;
use ecitygml_core::model::generics::GenericOccupiedSpace;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_generic_occupied_space(
    xml_document: &[u8],
) -> Result<GenericOccupiedSpace, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_occupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_occupied_space(xml_document, &spans),
        || de::from_reader::<_, GmlPlantCover>(xml_document).map_err(Error::from),
    );
    let abstract_occupied_space = abstract_occupied_space_result?;
    let parsed = parsed_result?;

    let mut generic_occupied_space =
        GenericOccupiedSpace::from_abstract_occupied_space(abstract_occupied_space);
    generic_occupied_space.set_class(parsed.class.map(Into::into));
    generic_occupied_space.set_functions(parsed.functions.into_iter().map(Into::into).collect());
    generic_occupied_space.set_usages(parsed.usages.into_iter().map(Into::into).collect());

    Ok(generic_occupied_space)
}

pub fn serialize_generic_occupied_space(
    generic_occupied_space: &GenericOccupiedSpace,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_occupied_space(
        generic_occupied_space.abstract_occupied_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlGenericOccupiedSpace::from(generic_occupied_space),
        formatting,
    )? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::GenericOccupiedSpace, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlGenericOccupiedSpace {
    #[serde(
        rename(serialize = "gen:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "gen:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "gen:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&GenericOccupiedSpace> for GmlGenericOccupiedSpace {
    fn from(item: &GenericOccupiedSpace) -> Self {
        Self {
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
        }
    }
}
