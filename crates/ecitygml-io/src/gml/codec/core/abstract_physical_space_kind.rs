use crate::Error;
use crate::gml::codec::core::abstract_occupied_space_kind::{
    deserialize_abstract_occupied_space_kind, serialize_abstract_occupied_space_kind,
};
use crate::gml::codec::core::abstract_unoccupied_space_kind::{
    deserialize_abstract_unoccupied_space_kind, serialize_abstract_unoccupied_space_kind,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::AbstractPhysicalSpaceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_physical_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractPhysicalSpaceKind>, Error> {
    if let Some(x) = deserialize_abstract_occupied_space_kind(xml_document, spans)? {
        return Ok(Some(AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(
            x,
        )));
    }

    if let Some(x) = deserialize_abstract_unoccupied_space_kind(xml_document, spans)? {
        return Ok(Some(
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x),
        ));
    }

    Ok(None)
}

pub fn serialize_abstract_physical_space_kind(
    abstract_physical_space_kind: &AbstractPhysicalSpaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_physical_space_kind {
        AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => {
            serialize_abstract_occupied_space_kind(x, formatting)
        }
        AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => {
            serialize_abstract_unoccupied_space_kind(x, formatting)
        }
    }
}
