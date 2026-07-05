use crate::Error;
use crate::gml::codec::water_body::abstract_water_boundary_surface::{
    deserialize_abstract_water_boundary_surface, serialize_abstract_water_boundary_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::water_body::{AsAbstractWaterBoundarySurface, WaterGroundSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_water_ground_surface(xml_document: &[u8]) -> Result<WaterGroundSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_water_boundary_surface =
        deserialize_abstract_water_boundary_surface(xml_document, &spans)?;
    let water_ground_surface =
        WaterGroundSurface::from_abstract_water_boundary_surface(abstract_water_boundary_surface);

    Ok(water_ground_surface)
}

pub fn serialize_water_ground_surface(
    water_ground_surface: &WaterGroundSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_water_boundary_surface(
        water_ground_surface.abstract_water_boundary_surface(),
        formatting,
    )?;

    Ok(XmlNode::new(XmlElement::WaterGroundSurface, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlWaterGroundSurface {}

impl From<&WaterGroundSurface> for GmlWaterGroundSurface {
    fn from(_item: &WaterGroundSurface) -> Self {
        Self {}
    }
}
