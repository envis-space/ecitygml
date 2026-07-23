use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::{
    deserialize_abstract_construction_surface, serialize_abstract_construction_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractConstructionSurface, OuterCeilingSurface};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};
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
    let xml_node_parts = serialize_abstract_construction_surface(
        outer_ceiling_surface.abstract_construction_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(
        CityGmlElement::OuterCeilingSurface.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlOuterCeilingSurface {}

impl From<&OuterCeilingSurface> for GmlOuterCeilingSurface {
    fn from(_item: &OuterCeilingSurface) -> Self {
        Self {}
    }
}
