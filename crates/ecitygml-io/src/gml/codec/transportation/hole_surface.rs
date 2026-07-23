use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use ecitygml_core::model::transportation::HoleSurface;
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};

pub fn deserialize_hole_surface(xml_document: &[u8]) -> Result<HoleSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document, &spans)?;
    let hole_surface = HoleSurface::from_abstract_thematic_surface(abstract_thematic_surface);

    Ok(hole_surface)
}

pub fn serialize_hole_surface(
    hole_surface: &HoleSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_thematic_surface(hole_surface.abstract_thematic_surface(), formatting)?;

    Ok(XmlNode::new(
        CityGmlElement::HoleSurface.into(),
        xml_node_parts,
    ))
}
