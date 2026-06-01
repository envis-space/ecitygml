use crate::Error;
use crate::gml::codec::core::deserialize_abstract_thematic_surface;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::construction::AbstractFillingSurface;

pub fn deserialize_abstract_filling_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractFillingSurface, Error> {
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document, spans)?;
    let abstract_filling_surface = AbstractFillingSurface::new(abstract_thematic_surface);

    Ok(abstract_filling_surface)
}
