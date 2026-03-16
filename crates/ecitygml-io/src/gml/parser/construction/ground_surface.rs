use crate::Error;
use crate::gml::parser::construction::abstract_construction_surface::deserialize_abstract_construction_surface;
use ecitygml_core::model::construction::GroundSurface;

pub fn deserialize_ground_surface(xml_document: &[u8]) -> Result<GroundSurface, Error> {
    let abstract_construction_surface = deserialize_abstract_construction_surface(xml_document)?;
    let ground_surface = GroundSurface::new(abstract_construction_surface);

    Ok(ground_surface)
}
