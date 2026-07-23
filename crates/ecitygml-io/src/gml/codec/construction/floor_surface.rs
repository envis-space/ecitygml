use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, FloorSurface};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};
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
    let xml_node_parts = serialize_abstract_construction_surface(
        floor_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(
        CityGmlElement::FloorSurface.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlFloorSurface {}

impl From<&FloorSurface> for GmlFloorSurface {
    fn from(_item: &FloorSurface) -> Self {
        Self {}
    }
}
