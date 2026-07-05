use crate::Error;
use crate::gml::codec::core::abstract_physical_space::{
    deserialize_abstract_physical_space, serialize_abstract_physical_space,
};
use crate::gml::util::{XmlElementSpans, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::{AbstractUnoccupiedSpace, AsAbstractPhysicalSpace};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_unoccupied_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractUnoccupiedSpace, Error> {
    let abstract_physical_space = deserialize_abstract_physical_space(xml_document, spans)?;
    let abstract_unoccupied_space =
        AbstractUnoccupiedSpace::from_abstract_physical_space(abstract_physical_space);

    Ok(abstract_unoccupied_space)
}

pub fn serialize_abstract_unoccupied_space(
    abstract_unoccupied_space: &AbstractUnoccupiedSpace,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_physical_space(
        abstract_unoccupied_space.abstract_physical_space(),
        formatting,
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractUnoccupiedSpace {}

impl From<&AbstractUnoccupiedSpace> for GmlAbstractUnoccupiedSpace {
    fn from(_item: &AbstractUnoccupiedSpace) -> Self {
        Self {}
    }
}
