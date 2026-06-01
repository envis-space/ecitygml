use crate::Error;
use crate::gml::codec::core::space_boundary_kind::deserialize_space_boundary_kind;
use crate::gml::codec::core::space_kind::deserialize_space_kind;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::CityObjectKind;

pub fn deserialize_city_object_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<CityObjectKind>, Error> {
    if let Some(x) = deserialize_space_kind(xml_document, spans)? {
        return Ok(Some(CityObjectKind::SpaceKind(x)));
    }

    if let Some(x) = deserialize_space_boundary_kind(xml_document, spans)? {
        return Ok(Some(CityObjectKind::SpaceBoundaryKind(x)));
    }

    Ok(None)
}
