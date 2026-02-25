use crate::Error;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::construction::GroundSurface;

pub fn parse_ground_surface(xml_document: &[u8]) -> Result<GroundSurface, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let ground_surface = GroundSurface::new(abstract_thematic_surface);

    Ok(ground_surface)
}
