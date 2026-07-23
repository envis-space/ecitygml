use crate::Error;
use crate::gml::codec::core::{deserialize_abstract_space, serialize_abstract_space};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::{AbstractLogicalSpace, AsAbstractSpace};
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeParts};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_logical_space(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractLogicalSpace, Error> {
    let abstract_space = deserialize_abstract_space(xml_document, spans)?;
    let abstract_logical_space = AbstractLogicalSpace::from_abstract_space(abstract_space);

    Ok(abstract_logical_space)
}

pub fn serialize_abstract_logical_space(
    abstract_logical_space: &AbstractLogicalSpace,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_space(abstract_logical_space.abstract_space(), formatting)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractLogicalSpace {}

impl From<&AbstractLogicalSpace> for GmlAbstractLogicalSpace {
    fn from(_item: &AbstractLogicalSpace) -> Self {
        Self {}
    }
}
