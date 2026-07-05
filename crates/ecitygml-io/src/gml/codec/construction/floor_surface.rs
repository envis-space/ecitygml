use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, FloorSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_floor_surface(xml_document: &[u8]) -> Result<FloorSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    Ok(FloorSurface::from_abstract_construction_surface(
        abstract_construction_surface,
    ))
}

pub fn serialize_floor_surface(
    floor_surface: &FloorSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let parts = serialize_abstract_construction_surface(
        floor_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(XmlElement::FloorSurface, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlFloorSurface {}

impl From<&FloorSurface> for GmlFloorSurface {
    fn from(_item: &FloorSurface) -> Self {
        Self {}
    }
}
