use crate::Error;
use crate::gml::codec::building::{
    deserialize_abstract_building_subdivision_kind, serialize_abstract_building_subdivision_kind,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::AbstractLogicalSpaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_logical_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractLogicalSpaceKind>, Error> {
    if let Some(x) = deserialize_abstract_building_subdivision_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_logical_space_kind(
    abstract_logical_space_kind: &AbstractLogicalSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_logical_space_kind {
        AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
            serialize_abstract_building_subdivision_kind(x, formatting)
        }
    }
}
