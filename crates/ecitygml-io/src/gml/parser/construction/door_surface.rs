use crate::Error;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::construction::DoorSurface;

pub fn parse_door_surface(xml_document: &[u8]) -> Result<DoorSurface, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let door_surface = DoorSurface::new(abstract_thematic_surface);

    Ok(door_surface)
}
