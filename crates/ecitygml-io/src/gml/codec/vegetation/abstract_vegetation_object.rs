use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::util::{XmlElementSpans, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AsAbstractOccupiedSpace;
use ecitygml_core::model::vegetation::AbstractVegetationObject;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_vegetation_object(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractVegetationObject, Error> {
    let occupied_space = deserialize_abstract_occupied_space(xml_document, spans)?;
    let abstract_vegetation_object =
        AbstractVegetationObject::from_abstract_occupied_space(occupied_space);

    Ok(abstract_vegetation_object)
}

pub fn serialize_abstract_vegetation_object(
    abstract_vegetation_object: &AbstractVegetationObject,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_occupied_space(
        abstract_vegetation_object.abstract_occupied_space(),
        formatting,
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractVegetationObject {}

impl From<&AbstractVegetationObject> for GmlAbstractVegetationObject {
    fn from(_item: &AbstractVegetationObject) -> Self {
        Self {}
    }
}
