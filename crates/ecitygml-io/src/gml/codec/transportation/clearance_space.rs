use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_unoccupied_space, serialize_abstract_unoccupied_space,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::core::AsAbstractUnoccupiedSpace;
use ecitygml_core::model::transportation::ClearanceSpace;
use ecitygml_core::model::transportation::values::ClearanceSpaceClassValue;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_clearance_space(xml_document: &[u8]) -> Result<ClearanceSpace, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_unoccupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_unoccupied_space(xml_document, &spans),
        || de::from_reader::<_, GmlClearanceSpace>(xml_document).map_err(Error::from),
    );
    let abstract_unoccupied_space = abstract_unoccupied_space_result?;
    let parsed = parsed_result?;

    let mut clearance_space =
        ClearanceSpace::from_abstract_unoccupied_space(abstract_unoccupied_space);

    clearance_space.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(ClearanceSpaceClassValue::from),
    );

    Ok(clearance_space)
}

pub fn serialize_clearance_space(
    clearance_space: &ClearanceSpace,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_unoccupied_space(
        clearance_space.abstract_unoccupied_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(GmlClearanceSpace::from(clearance_space), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::ClearanceSpace.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlClearanceSpace {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,
}

impl From<&ClearanceSpace> for GmlClearanceSpace {
    fn from(item: &ClearanceSpace) -> Self {
        Self {
            class: item
                .class()
                .map(ClearanceSpaceClassValue::code)
                .map(Into::into),
        }
    }
}
