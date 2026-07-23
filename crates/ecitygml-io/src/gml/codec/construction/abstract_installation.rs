use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::construction::AbstractInstallation;
use ecitygml_core::model::core::AsAbstractOccupiedSpace;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeParts};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_installation(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractInstallation, Error> {
    let abstract_occupied_space = deserialize_abstract_occupied_space(xml_document, spans)?;
    let abstract_installation =
        AbstractInstallation::from_abstract_occupied_space(abstract_occupied_space);

    Ok(abstract_installation)
}

pub fn serialize_abstract_installation(
    abstract_installation: &AbstractInstallation,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_occupied_space(abstract_installation.abstract_occupied_space(), formatting)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractInstallation {}

impl From<&AbstractInstallation> for GmlAbstractInstallation {
    fn from(_item: &AbstractInstallation) -> Self {
        Self {}
    }
}
