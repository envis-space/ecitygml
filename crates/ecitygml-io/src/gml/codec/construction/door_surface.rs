use crate::Error;
use crate::gml::codec::core::deserialize_abstract_filling_surface;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::construction::DoorSurface;

pub fn deserialize_door_surface(xml_document: &[u8]) -> Result<DoorSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_surface = deserialize_abstract_filling_surface(xml_document, &spans)?;
    let door_surface = DoorSurface::new(abstract_filling_surface);

    Ok(door_surface)
}
