use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use ecitygml_core::model::water_body::AbstractWaterBoundarySurface;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeParts};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_water_boundary_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractWaterBoundarySurface, Error> {
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document, spans)?;
    let abstract_water_boundary_surface =
        AbstractWaterBoundarySurface::from_abstract_thematic_surface(abstract_thematic_surface);

    Ok(abstract_water_boundary_surface)
}

pub fn serialize_abstract_water_boundary_surface(
    abstract_water_boundary_surface: &AbstractWaterBoundarySurface,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let xml_node_parts = serialize_abstract_thematic_surface(
        abstract_water_boundary_surface.abstract_thematic_surface(),
        formatting,
    )?;

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractWaterBoundarySurface {}

impl From<&AbstractWaterBoundarySurface> for GmlAbstractWaterBoundarySurface {
    fn from(_item: &AbstractWaterBoundarySurface) -> Self {
        Self {}
    }
}
