use crate::Error;
use crate::gml::codec::core::deserialize_abstract_city_object;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::AbstractSpaceBoundary;

pub fn deserialize_abstract_space_boundary(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractSpaceBoundary, Error> {
    let abstract_city_object = deserialize_abstract_city_object(xml_document, spans)?;
    let abstract_space_boundary = AbstractSpaceBoundary::new(abstract_city_object);

    Ok(abstract_space_boundary)
}
