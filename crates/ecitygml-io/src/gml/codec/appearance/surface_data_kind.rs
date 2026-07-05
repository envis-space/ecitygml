use crate::Error;
use crate::gml::codec::appearance::texture_kind::{
    deserialize_texture_kind, serialize_texture_kind,
};
use crate::gml::codec::appearance::x3d_material::{
    deserialize_x3d_material, serialize_x3d_material,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::appearance::SurfaceDataKind;

pub fn deserialize_surface_data_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<SurfaceDataKind>, Error> {
    if let Some(x) = deserialize_texture_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(XmlElement::X3DMaterial) {
        let x3d_material = deserialize_x3d_material(&xml_document[span.start..span.end])?;
        return Ok(Some(x3d_material.into()));
    }

    Ok(None)
}

pub fn serialize_surface_data_kind(
    surface_data_kind: &SurfaceDataKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match surface_data_kind {
        SurfaceDataKind::TextureKind(x) => serialize_texture_kind(x, formatting),
        SurfaceDataKind::X3DMaterial(x) => serialize_x3d_material(x, formatting),
    }
}
