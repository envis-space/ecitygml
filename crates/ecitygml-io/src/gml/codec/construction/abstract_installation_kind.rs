use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_installation, serialize_building_installation,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::AbstractInstallationKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_installation_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractInstallationKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::BuildingInstallation.into()) {
        let building_installation =
            deserialize_building_installation(&xml_document[span.start..span.end])?;
        return Ok(Some(building_installation.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_installation_kind(
    abstract_installation_kind: &AbstractInstallationKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_installation_kind {
        AbstractInstallationKind::BuildingInstallation(x) => {
            serialize_building_installation(x, formatting)
        }
    }
}
