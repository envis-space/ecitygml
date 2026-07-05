use crate::Error;
use crate::gml::codec::vegetation::{
    deserialize_plant_cover, deserialize_solitary_vegetation_object, serialize_plant_cover,
    serialize_solitary_vegetation_object,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::vegetation::VegetationObjectKind;

pub fn deserialize_vegetation_object_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<VegetationObjectKind>, Error> {
    if let Some(span) = spans.first(XmlElement::PlantCover) {
        let plant_cover = deserialize_plant_cover(&xml_document[span.start..span.end])?;
        return Ok(Some(plant_cover.into()));
    }

    if let Some(span) = spans.first(XmlElement::SolitaryVegetationObject) {
        let solitary_vegetation_object =
            deserialize_solitary_vegetation_object(&xml_document[span.start..span.end])?;
        return Ok(Some(solitary_vegetation_object.into()));
    }

    Ok(None)
}

pub fn serialize_vegetation_object_kind(
    vegetation_object_kind: &VegetationObjectKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match vegetation_object_kind {
        VegetationObjectKind::PlantCover(x) => serialize_plant_cover(x, formatting),
        VegetationObjectKind::SolitaryVegetationObject(x) => {
            serialize_solitary_vegetation_object(x, formatting)
        }
    }
}
