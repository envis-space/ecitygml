use crate::Error;
use crate::gml::parser::core::deserialize_abstract_thematic_surface;
use ecitygml_core::model::construction::AbstractConstructionSurface;

pub fn deserialize_abstract_construction_surface(
    xml_document: &[u8],
) -> Result<AbstractConstructionSurface, Error> {
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document)?;
    let abstract_construction_surface = AbstractConstructionSurface::new(abstract_thematic_surface);

    Ok(abstract_construction_surface)
}
