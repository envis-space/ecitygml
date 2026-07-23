use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, OuterFloorSurface};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};
use serde::{Deserialize, Serialize};

pub fn deserialize_outer_floor_surface(xml_document: &[u8]) -> Result<OuterFloorSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    Ok(OuterFloorSurface::from_abstract_construction_surface(
        abstract_construction_surface,
    ))
}

pub fn serialize_outer_floor_surface(
    outer_floor_surface: &OuterFloorSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_construction_surface(
        outer_floor_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(
        CityGmlElement::OuterFloorSurface.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlOuterFloorSurface {}

impl From<&OuterFloorSurface> for GmlOuterFloorSurface {
    fn from(_item: &OuterFloorSurface) -> Self {
        Self {}
    }
}
