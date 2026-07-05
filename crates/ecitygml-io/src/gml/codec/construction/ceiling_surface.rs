use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, CeilingSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_ceiling_surface(xml_document: &[u8]) -> Result<CeilingSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    Ok(CeilingSurface::from_abstract_construction_surface(
        abstract_construction_surface,
    ))
}

pub fn serialize_ceiling_surface(
    ceiling_surface: &CeilingSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let parts = serialize_abstract_construction_surface(
        ceiling_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(XmlElement::CeilingSurface, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCeilingSurface {}

impl From<&CeilingSurface> for GmlCeilingSurface {
    fn from(_item: &CeilingSurface) -> Self {
        Self {}
    }
}
