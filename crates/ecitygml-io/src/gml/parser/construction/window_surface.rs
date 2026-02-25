use crate::Error;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::construction::WindowSurface;

pub fn parse_window_surface(xml_document: &[u8]) -> Result<WindowSurface, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let window_surface = WindowSurface::new(abstract_thematic_surface);

    Ok(window_surface)
}
