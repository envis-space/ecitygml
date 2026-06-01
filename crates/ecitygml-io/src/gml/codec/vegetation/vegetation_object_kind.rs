use crate::Error;
use crate::gml::codec::vegetation::deserialize_solitary_vegetation_object;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::vegetation::VegetationObjectKind;

pub fn deserialize_vegetation_object_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<VegetationObjectKind>, Error> {
    if let Some(span) = spans.first(XmlElement::SolitaryVegetationObject) {
        let solitary_vegetation_object =
            deserialize_solitary_vegetation_object(&xml_document[span.start..span.end])?;
        return Ok(Some(VegetationObjectKind::SolitaryVegetationObject(
            solitary_vegetation_object,
        )));
    }

    Ok(None)
}
