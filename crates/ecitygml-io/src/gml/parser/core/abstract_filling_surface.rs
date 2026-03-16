use crate::Error;
use crate::gml::parser::core::deserialize_abstract_thematic_surface;
use ecitygml_core::model::construction::AbstractFillingSurface;

pub fn deserialize_abstract_filling_surface(
    xml_document: &[u8],
) -> Result<AbstractFillingSurface, Error> {
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document)?;
    let abstract_filling_surface = AbstractFillingSurface::new(abstract_thematic_surface);

    Ok(abstract_filling_surface)
}
