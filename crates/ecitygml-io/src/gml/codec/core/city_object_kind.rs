use crate::Error;
use crate::gml::codec::core::space_boundary_kind::{
    deserialize_space_boundary_kind, serialize_space_boundary_kind,
};
use crate::gml::codec::core::space_kind::{deserialize_space_kind, serialize_space_kind};
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::CityObjectKind;

pub fn deserialize_city_object_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<CityObjectKind>, Error> {
    if let Some(x) = deserialize_space_boundary_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_space_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_city_object_kind(
    city_object_kind: &CityObjectKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match city_object_kind {
        CityObjectKind::SpaceBoundaryKind(x) => serialize_space_boundary_kind(x, formatting),
        CityObjectKind::SpaceKind(x) => serialize_space_kind(x, formatting),
    }
}
