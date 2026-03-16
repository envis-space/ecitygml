use crate::Error;
use crate::gml::parser::construction::abstract_construction_surface::deserialize_abstract_construction_surface;
use ecitygml_core::model::construction::RoofSurface;

pub fn deserialize_roof_surface(xml_document: &[u8]) -> Result<RoofSurface, Error> {
    let abstract_construction_surface = deserialize_abstract_construction_surface(xml_document)?;
    let roof_surface = RoofSurface::new(abstract_construction_surface);

    Ok(roof_surface)
}
