use crate::Error;
use crate::gml::codec::appearance::georeferenced_texture::{
    deserialize_georeferenced_texture, serialize_georeferenced_texture,
};
use crate::gml::codec::appearance::parameterized_texture::{
    deserialize_parameterized_texture, serialize_parameterized_texture,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::appearance::AbstractTextureKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_texture_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractTextureKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::GeoreferencedTexture.into()) {
        let georeferenced_texture =
            deserialize_georeferenced_texture(&xml_document[span.start..span.end])?;
        return Ok(Some(georeferenced_texture.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::ParameterizedTexture.into()) {
        let parameterized_texture =
            deserialize_parameterized_texture(&xml_document[span.start..span.end])?;
        return Ok(Some(parameterized_texture.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_texture_kind(
    abstract_texture_kind: &AbstractTextureKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_texture_kind {
        AbstractTextureKind::GeoreferencedTexture(x) => {
            serialize_georeferenced_texture(x, formatting)
        }
        AbstractTextureKind::ParameterizedTexture(x) => {
            serialize_parameterized_texture(x, formatting)
        }
    }
}
