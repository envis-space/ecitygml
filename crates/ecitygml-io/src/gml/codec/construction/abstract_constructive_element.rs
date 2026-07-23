use crate::Error;
use crate::gml::codec::construction::abstract_filling_element_property::{
    deserialize_abstract_filling_element_property, serialize_abstract_filling_element_property,
};
use crate::gml::codec::core::{
    deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use ecitygml_core::model::core::AsAbstractOccupiedSpace;
use egml::io::util::collect_children;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_constructive_element(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractConstructiveElement, Error> {
    let mut abstract_occupied_space_result = None;
    let mut filling_elements_result = None;
    let mut parsed_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_occupied_space_result =
                Some(deserialize_abstract_occupied_space(xml_document, spans));
        });
        s.spawn(|_| {
            filling_elements_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::AbstractFillingElementProperty.into(),
                deserialize_abstract_filling_element_property,
            ));
        });
        s.spawn(|_| {
            parsed_result = Some(
                de::from_reader::<_, GmlAbstractConstructiveElement>(xml_document)
                    .map_err(Error::from),
            );
        });
    });

    let abstract_occupied_space =
        abstract_occupied_space_result.expect("rayon::scope guarantees all spawns complete")?;
    let filling_elements =
        filling_elements_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_constructive_element =
        AbstractConstructiveElement::from_abstract_occupied_space(abstract_occupied_space);
    abstract_constructive_element.set_fillings(filling_elements);
    abstract_constructive_element.set_is_structural_element(parsed.is_structural_element);

    Ok(abstract_constructive_element)
}

pub fn serialize_abstract_constructive_element(
    abstract_constructive_element: &AbstractConstructiveElement,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_occupied_space(
        abstract_constructive_element.abstract_occupied_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractConstructiveElement::from(abstract_constructive_element),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for prop in abstract_constructive_element.fillings() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_abstract_filling_element_property(prop, formatting)?,
        ));
    }

    Ok(xml_node_parts)
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
            is_structural_element: item.is_structural_element(),
        }
    }
}
