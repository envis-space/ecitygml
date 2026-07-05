use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::util::{XmlElementSpans, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::AbstractFillingElement;
use ecitygml_core::model::core::AsAbstractOccupiedSpace;

pub fn deserialize_abstract_filling_element(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractFillingElement, Error> {
    let abstract_occupied_space = deserialize_abstract_occupied_space(xml_document, spans)?;
    let window_surface =
        AbstractFillingElement::from_abstract_occupied_space(abstract_occupied_space);

    Ok(window_surface)
}

pub fn serialize_abstract_filling_element(
    abstract_filling_element: &AbstractFillingElement,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_occupied_space(
        abstract_filling_element.abstract_occupied_space(),
        formatting,
    )
}
