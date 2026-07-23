use crate::Error;
use crate::gml::codec::appearance::abstract_texture_kind::{
    deserialize_abstract_texture_kind, serialize_abstract_texture_kind,
};
use crate::gml::codec::appearance::x3d_material::{
    deserialize_x3d_material, serialize_x3d_material,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::appearance::AbstractSurfaceDataKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_surface_data_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractSurfaceDataKind>, Error> {
    if let Some(x) = deserialize_abstract_texture_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }
    if let Some(span) = spans.first(CityGmlElement::X3DMaterial.into()) {
        let x3d_material = deserialize_x3d_material(&xml_document[span.start..span.end])?;
        return Ok(Some(x3d_material.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_surface_data_kind(
    abstract_surface_data_kind: &AbstractSurfaceDataKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_surface_data_kind {
        AbstractSurfaceDataKind::AbstractTextureKind(x) => {
            serialize_abstract_texture_kind(x, formatting)
        }
        AbstractSurfaceDataKind::X3DMaterial(x) => serialize_x3d_material(x, formatting),
    }
}
