use crate::Error;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::construction::RoofSurface;

pub fn parse_roof_surface(xml_document: &[u8]) -> Result<RoofSurface, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let roof_surface = RoofSurface::new(abstract_thematic_surface);

    Ok(roof_surface)
}
