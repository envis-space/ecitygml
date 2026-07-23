use crate::Error;
use crate::gml::codec::vegetation::{
    deserialize_plant_cover, deserialize_solitary_vegetation_object, serialize_plant_cover,
    serialize_solitary_vegetation_object,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::vegetation::AbstractVegetationObjectKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_vegetation_object_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractVegetationObjectKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::PlantCover.into()) {
        let plant_cover = deserialize_plant_cover(&xml_document[span.start..span.end])?;
        return Ok(Some(plant_cover.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::SolitaryVegetationObject.into()) {
        let solitary_vegetation_object =
            deserialize_solitary_vegetation_object(&xml_document[span.start..span.end])?;
        return Ok(Some(solitary_vegetation_object.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_vegetation_object_kind(
    abstract_vegetation_object_kind: &AbstractVegetationObjectKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_vegetation_object_kind {
        AbstractVegetationObjectKind::PlantCover(x) => serialize_plant_cover(x, formatting),
        AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
            serialize_solitary_vegetation_object(x, formatting)
        }
    }
}
