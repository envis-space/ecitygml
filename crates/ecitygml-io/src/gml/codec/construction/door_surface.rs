use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_filling_surface, serialize_abstract_filling_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractFillingSurface, DoorSurface};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};
use serde::{Deserialize, Serialize};

pub fn deserialize_door_surface(xml_document: &[u8]) -> Result<DoorSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_surface = deserialize_abstract_filling_surface(xml_document, &spans)?;
    let door_surface = DoorSurface::from_abstract_filling_surface(abstract_filling_surface);

    Ok(door_surface)
}

pub fn serialize_door_surface(
    door_surface: &DoorSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_filling_surface(door_surface.abstract_filling_surface(), formatting)?;

    Ok(XmlNode::new(
        CityGmlElement::DoorSurface.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlDoorSurface {}

impl From<&DoorSurface> for GmlDoorSurface {
    fn from(_item: &DoorSurface) -> Self {
        Self {}
    }
}
