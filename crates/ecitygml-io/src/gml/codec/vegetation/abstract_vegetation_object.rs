use crate::Error;
use crate::gml::codec::core::deserialize_abstract_occupied_space;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::vegetation::AbstractVegetationObject;

pub fn deserialize_abstract_vegetation_object(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractVegetationObject, Error> {
    let occupied_space = deserialize_abstract_occupied_space(xml_document, spans)?;
    let abstract_vegetation_object = AbstractVegetationObject::new(occupied_space);

    Ok(abstract_vegetation_object)
}
