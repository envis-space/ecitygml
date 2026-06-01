use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::deserialize_abstract_construction_surface;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::construction::GroundSurface;

pub fn deserialize_ground_surface(xml_document: &[u8]) -> Result<GroundSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    let ground_surface = GroundSurface::new(abstract_construction_surface);

    Ok(ground_surface)
}
