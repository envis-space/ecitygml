use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, OuterCeilingSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_outer_ceiling_surface(
    xml_document: &[u8],
) -> Result<OuterCeilingSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    Ok(OuterCeilingSurface::from_abstract_construction_surface(
        abstract_construction_surface,
    ))
}

pub fn serialize_outer_ceiling_surface(
    outer_ceiling_surface: &OuterCeilingSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let parts = serialize_abstract_construction_surface(
        outer_ceiling_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(XmlElement::OuterCeilingSurface, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlOuterCeilingSurface {}

impl From<&OuterCeilingSurface> for GmlOuterCeilingSurface {
    fn from(_item: &OuterCeilingSurface) -> Self {
        Self {}
    }
}
