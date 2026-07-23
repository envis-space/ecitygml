use crate::Error;
use crate::gml::codec::appearance::{deserialize_appearance, serialize_appearance};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractAppearanceKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_appearance_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractAppearanceKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::Appearance.into()) {
        let appearance = deserialize_appearance(&xml_document[span.start..span.end])?;
        return Ok(Some(appearance.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_appearance_kind(
    abstract_appearance_kind: &AbstractAppearanceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_appearance_kind {
        AbstractAppearanceKind::Appearance(x) => serialize_appearance(x, formatting),
    }
}
