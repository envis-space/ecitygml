use crate::Error;
use crate::gml::codec::appearance::georeferenced_texture::{
    deserialize_georeferenced_texture, serialize_georeferenced_texture,
};
use crate::gml::codec::appearance::parameterized_texture::{
    deserialize_parameterized_texture, serialize_parameterized_texture,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::appearance::TextureKind;

pub fn deserialize_texture_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<TextureKind>, Error> {
    if let Some(span) = spans.first(XmlElement::GeoreferencedTexture) {
        let georeferenced_texture =
            deserialize_georeferenced_texture(&xml_document[span.start..span.end])?;
        return Ok(Some(georeferenced_texture.into()));
    }
    if let Some(span) = spans.first(XmlElement::ParameterizedTexture) {
        let parameterized_texture =
            deserialize_parameterized_texture(&xml_document[span.start..span.end])?;
        return Ok(Some(parameterized_texture.into()));
    }

    Ok(None)
}

pub fn serialize_texture_kind(
    texture_kind: &TextureKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match texture_kind {
        TextureKind::GeoreferencedTexture(x) => serialize_georeferenced_texture(x, formatting),
        TextureKind::ParameterizedTexture(x) => serialize_parameterized_texture(x, formatting),
    }
}
