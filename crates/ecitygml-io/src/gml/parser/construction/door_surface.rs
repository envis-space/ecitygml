use crate::Error;
use crate::gml::parser::core::deserialize_abstract_filling_surface;
use ecitygml_core::model::construction::DoorSurface;

pub fn deserialize_door_surface(xml_document: &[u8]) -> Result<DoorSurface, Error> {
    let abstract_filling_surface = deserialize_abstract_filling_surface(xml_document)?;
    let door_surface = DoorSurface::new(abstract_filling_surface);

    Ok(door_surface)
}
