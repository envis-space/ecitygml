use crate::Error;

use crate::gml::util::{XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use crate::gml::write::Formatting;

use crate::gml::codec::appearance::abstract_surface_data::{
    deserialize_abstract_surface_data, serialize_abstract_surface_data,
};
use crate::gml::codec::appearance::basic_types::GmlColorPlusOpacity;
use crate::gml::codec::appearance::enums::{GmlTextureType, GmlWrapMode};
use ecitygml_core::model::appearance::{
    AbstractTexture, AsAbstractSurfaceData, AsAbstractTexture, AsAbstractTextureMut,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_texture(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractTexture, Error> {
    let (abstract_surface_data_result, gml_result) = rayon::join(
        || deserialize_abstract_surface_data(xml_document, spans),
        || de::from_reader::<_, GmlAbstractTexture>(xml_document).map_err(Error::from),
    );
    let abstract_feature = abstract_surface_data_result?;
    let gml = gml_result?;

    let mut abstract_texture = AbstractTexture::from_abstract_surface_data(abstract_feature);
    abstract_texture.set_image_uri(gml.image_uri);
    abstract_texture.set_texture_type(gml.texture_type.map(Into::into));
    abstract_texture.set_wrap_mode(gml.wrap_mode.map(Into::into));
    abstract_texture.set_border_color(gml.border_color.map(TryInto::try_into).transpose()?);

    Ok(abstract_texture)
}

pub fn serialize_abstract_texture(
    abstract_texture: &AbstractTexture,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts =
        serialize_abstract_surface_data(abstract_texture.abstract_surface_data(), formatting)?;

    if let Some(raw) = serialize_inner(GmlAbstractTexture::from(abstract_texture), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractTexture {
    #[serde(
        rename(serialize = "app:imageURI", deserialize = "imageURI"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_uri: Option<String>,

    #[serde(
        rename(serialize = "app:textureType", deserialize = "textureType"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub texture_type: Option<GmlTextureType>,

    #[serde(
        rename(serialize = "app:wrapMode", deserialize = "wrapMode"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wrap_mode: Option<GmlWrapMode>,

    #[serde(
        rename(serialize = "app:borderColor", deserialize = "borderColor"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub border_color: Option<GmlColorPlusOpacity>,
}

impl From<&AbstractTexture> for GmlAbstractTexture {
    fn from(item: &AbstractTexture) -> Self {
        Self {
            image_uri: item.image_uri().map(Into::into),
            texture_type: item.texture_type().map(Into::into),
            wrap_mode: item.wrap_mode().map(Into::into),
            border_color: item.border_color().map(Into::into),
        }
    }
}
