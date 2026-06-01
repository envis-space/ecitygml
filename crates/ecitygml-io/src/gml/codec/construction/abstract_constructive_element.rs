use crate::Error;
use crate::gml::codec::construction::filling_element_property::deserialize_filling_element_property;
use crate::gml::codec::core::deserialize_abstract_occupied_space;
use crate::gml::util::{XmlElement, XmlElementSpans, collect_children};
use ecitygml_core::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_constructive_element(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractConstructiveElement, Error> {
    let abstract_occupied_space = deserialize_abstract_occupied_space(xml_document, spans)?;
    let mut abstract_constructive_element =
        AbstractConstructiveElement::new(abstract_occupied_space);

    let filling_elements = collect_children(
        xml_document,
        spans,
        XmlElement::FillingElementProperty,
        deserialize_filling_element_property,
    )?;
    abstract_constructive_element.set_fillings(filling_elements);

    let parsed: GmlAbstractConstructiveElement =
        de::from_reader::<_, GmlAbstractConstructiveElement>(xml_document)?;
    abstract_constructive_element.set_is_structural_element(parsed.is_structural_element);

    Ok(abstract_constructive_element)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractConstructiveElement {
    #[serde(
        rename(
            serialize = "con:isStructuralElement",
            deserialize = "isStructuralElement"
        ),
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub is_structural_element: Option<bool>,
}

impl From<&AbstractConstructiveElement> for GmlAbstractConstructiveElement {
    fn from(item: &AbstractConstructiveElement) -> Self {
        Self {
            is_structural_element: *item.is_structural_element(),
        }
    }
}
