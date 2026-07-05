use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, InteriorWallSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_interior_wall_surface(
    xml_document: &[u8],
) -> Result<InteriorWallSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    Ok(InteriorWallSurface::from_abstract_construction_surface(
        abstract_construction_surface,
    ))
}

pub fn serialize_interior_wall_surface(
    interior_wall_surface: &InteriorWallSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let parts = serialize_abstract_construction_surface(
        interior_wall_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(XmlElement::InteriorWallSurface, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlInteriorWallSurface {}

impl From<&InteriorWallSurface> for GmlInteriorWallSurface {
    fn from(_item: &InteriorWallSurface) -> Self {
        Self {}
    }
}
