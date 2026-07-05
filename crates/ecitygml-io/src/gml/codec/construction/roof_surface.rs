use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, RoofSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_roof_surface(xml_document: &[u8]) -> Result<RoofSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    let roof_surface =
        RoofSurface::from_abstract_construction_surface(abstract_construction_surface);

    Ok(roof_surface)
}

pub fn serialize_roof_surface(
    roof_surface: &RoofSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_construction_surface(
        roof_surface.abstract_construction_surface(),
        formatting,
    )?;

    Ok(XmlNode::new(XmlElement::RoofSurface, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlRoofSurface {}

impl From<&RoofSurface> for GmlRoofSurface {
    fn from(_item: &RoofSurface) -> Self {
        Self {}
    }
}
