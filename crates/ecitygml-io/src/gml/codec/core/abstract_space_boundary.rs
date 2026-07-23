use crate::Error;
use crate::gml::codec::core::{deserialize_abstract_city_object, serialize_abstract_city_object};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::{AbstractSpaceBoundary, AsAbstractCityObject};
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeParts};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_space_boundary(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractSpaceBoundary, Error> {
    let abstract_city_object = deserialize_abstract_city_object(xml_document, spans)?;
    let abstract_space_boundary =
        AbstractSpaceBoundary::from_abstract_city_object(abstract_city_object);

    Ok(abstract_space_boundary)
}

pub fn serialize_abstract_space_boundary(
    abstract_space_boundary: &AbstractSpaceBoundary,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_city_object(abstract_space_boundary.abstract_city_object(), formatting)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSpaceBoundary {}

impl From<&AbstractSpaceBoundary> for GmlAbstractSpaceBoundary {
    fn from(_item: &AbstractSpaceBoundary) -> Self {
        Self {}
    }
}
