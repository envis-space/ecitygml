use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, GroundSurface};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};
use serde::{Deserialize, Serialize};

pub fn deserialize_ground_surface(xml_document: &[u8]) -> Result<GroundSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    let ground_surface =
        GroundSurface::from_abstract_construction_surface(abstract_construction_surface);

    Ok(ground_surface)
}

pub fn serialize_ground_surface(
    ground_surface: &GroundSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_construction_surface(
        ground_surface.abstract_construction_surface(),
        formatting,
    )?;

    Ok(XmlNode::new(
        CityGmlElement::GroundSurface.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlGroundSurface {}

impl From<&GroundSurface> for GmlGroundSurface {
    fn from(_item: &GroundSurface) -> Self {
        Self {}
    }
}
