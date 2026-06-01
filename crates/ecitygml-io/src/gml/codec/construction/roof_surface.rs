use crate::Error;
use crate::gml::codec::construction::abstract_construction_surface::deserialize_abstract_construction_surface;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::construction::RoofSurface;

pub fn deserialize_roof_surface(xml_document: &[u8]) -> Result<RoofSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_construction_surface =
        deserialize_abstract_construction_surface(xml_document, &spans)?;
    let roof_surface = RoofSurface::new(abstract_construction_surface);

    Ok(roof_surface)
}
