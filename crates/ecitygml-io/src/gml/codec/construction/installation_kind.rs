use crate::Error;
use crate::gml::codec::building::{
    deserialize_building_installation, serialize_building_installation,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::InstallationKind;

pub fn deserialize_installation_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<InstallationKind>, Error> {
    if let Some(span) = spans.first(XmlElement::BuildingInstallation) {
        let building_installation =
            deserialize_building_installation(&xml_document[span.start..span.end])?;
        return Ok(Some(building_installation.into()));
    }

    Ok(None)
}

pub fn serialize_installation_kind(
    installation_kind: &InstallationKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match installation_kind {
        InstallationKind::BuildingInstallation(x) => serialize_building_installation(x, formatting),
    }
}
