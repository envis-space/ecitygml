use crate::Error;
use crate::gml::codec::core::physical_space_kind::deserialize_physical_space_kind;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::SpaceKind;

pub fn deserialize_space_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<SpaceKind>, Error> {
    if let Some(x) = deserialize_physical_space_kind(xml_document, spans)? {
        return Ok(Some(SpaceKind::PhysicalSpaceKind(x)));
    }

    Ok(None)
}
