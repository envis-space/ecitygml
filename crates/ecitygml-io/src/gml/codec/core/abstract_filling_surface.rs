use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::{XmlElementSpans, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::AbstractFillingSurface;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_filling_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractFillingSurface, Error> {
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document, spans)?;
    let abstract_filling_surface =
        AbstractFillingSurface::from_abstract_thematic_surface(abstract_thematic_surface);

    Ok(abstract_filling_surface)
}

pub fn serialize_abstract_filling_surface(
    abstract_filling_surface: &AbstractFillingSurface,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let xml_node_parts = serialize_abstract_thematic_surface(
        abstract_filling_surface.abstract_thematic_surface(),
        formatting,
    )?;

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractFillingSurface {}

impl From<&AbstractFillingSurface> for GmlAbstractFillingSurface {
    fn from(_item: &AbstractFillingSurface) -> Self {
        Self {}
    }
}
