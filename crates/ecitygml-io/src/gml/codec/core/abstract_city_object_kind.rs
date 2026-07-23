use crate::Error;
use crate::gml::codec::core::abstract_space_boundary_kind::{
    deserialize_abstract_space_boundary_kind, serialize_abstract_space_boundary_kind,
};
use crate::gml::codec::core::abstract_space_kind::{
    deserialize_abstract_space_kind, serialize_abstract_space_kind,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::AbstractCityObjectKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_city_object_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractCityObjectKind>, Error> {
    if let Some(x) = deserialize_abstract_space_boundary_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_city_object_kind(
    abstract_city_object_kind: &AbstractCityObjectKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_city_object_kind {
        AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => {
            serialize_abstract_space_boundary_kind(x, formatting)
        }
        AbstractCityObjectKind::AbstractSpaceKind(x) => {
            serialize_abstract_space_kind(x, formatting)
        }
    }
}
