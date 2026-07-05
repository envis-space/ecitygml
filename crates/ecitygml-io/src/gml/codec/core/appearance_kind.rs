use crate::gml::codec::appearance::{deserialize_appearance, serialize_appearance};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::{Error, Formatting};
use ecitygml_core::model::core::AppearanceKind;

pub fn deserialize_appearance_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<AppearanceKind>, Error> {
    if let Some(span) = spans.first(XmlElement::Appearance) {
        let appearance = deserialize_appearance(&xml_document[span.start..span.end])?;
        return Ok(Some(appearance.into()));
    }

    Ok(None)
}

pub fn serialize_appearance_kind(
    appearance_kind: &AppearanceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match appearance_kind {
        AppearanceKind::Appearance(x) => serialize_appearance(x, formatting),
    }
}
