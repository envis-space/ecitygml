use crate::Error;
use crate::gml::codec::appearance::abstract_surface_data::{
    deserialize_abstract_surface_data, serialize_abstract_surface_data,
};
use crate::gml::codec::appearance::basic_types::{GmlColor, GmlGeometryReference};
use crate::gml::codec::core::basic_types::GmlDoubleBetween0And1;
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::appearance::{AsAbstractSurfaceData, X3DMaterial};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_x3d_material(xml_document: &[u8]) -> Result<X3DMaterial, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_surface_data_result, gml_result) = rayon::join(
        || deserialize_abstract_surface_data(xml_document, &spans),
        || de::from_reader::<_, GmlX3DMaterial>(xml_document).map_err(Error::from),
    );
    let abstract_surface_data = abstract_surface_data_result?;
    let gml = gml_result?;

    let mut x3d_material = X3DMaterial::from_abstract_surface_data(abstract_surface_data);
    x3d_material.set_ambient_intensity(gml.ambient_intensity.map(TryInto::try_into).transpose()?);
    x3d_material.set_diffuse_color(gml.diffuse_color.map(TryInto::try_into).transpose()?);
    x3d_material.set_emissive_color(gml.emissive_color.map(TryInto::try_into).transpose()?);
    x3d_material.set_specular_color(gml.specular_color.map(TryInto::try_into).transpose()?);
    x3d_material.set_shininess(gml.shininess.map(TryInto::try_into).transpose()?);
    x3d_material.set_transparency(gml.transparency.map(TryInto::try_into).transpose()?);
    x3d_material.set_is_smooth(gml.is_smooth);
    x3d_material.set_targets(gml.targets.into_iter().map(Into::into).collect());

    Ok(x3d_material)
}

pub fn serialize_x3d_material(
    x3d_material: &X3DMaterial,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_surface_data(x3d_material.abstract_surface_data(), formatting)?;

    if let Some(raw) = serialize_inner(GmlX3DMaterial::from(x3d_material), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::X3DMaterial, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlX3DMaterial {
    #[serde(
        rename(serialize = "app:ambientIntensity", deserialize = "ambientIntensity"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ambient_intensity: Option<GmlDoubleBetween0And1>,

    #[serde(
        rename(serialize = "app:diffuseColor", deserialize = "diffuseColor"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub diffuse_color: Option<GmlColor>,

    #[serde(
        rename(serialize = "app:emissiveColor", deserialize = "emissiveColor"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emissive_color: Option<GmlColor>,

    #[serde(
        rename(serialize = "app:specularColor", deserialize = "specularColor"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub specular_color: Option<GmlColor>,

    #[serde(
        rename(serialize = "app:shininess", deserialize = "shininess"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shininess: Option<GmlDoubleBetween0And1>,

    #[serde(
        rename(serialize = "app:transparency", deserialize = "transparency"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transparency: Option<GmlDoubleBetween0And1>,

    #[serde(
        rename(serialize = "app:isSmooth", deserialize = "isSmooth"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_smooth: Option<bool>,

    #[serde(rename(serialize = "app:target", deserialize = "target"), default)]
    pub targets: Vec<GmlGeometryReference>,
}

impl From<&X3DMaterial> for GmlX3DMaterial {
    fn from(item: &X3DMaterial) -> Self {
        Self {
            ambient_intensity: item.ambient_intensity().map(Into::into),
            diffuse_color: item.diffuse_color().map(Into::into),
            emissive_color: item.emissive_color().map(Into::into),
            specular_color: item.specular_color().map(Into::into),
            shininess: item.shininess().map(Into::into),
            transparency: item.transparency().map(Into::into),
            is_smooth: item.is_smooth(),
            targets: item.targets().iter().map(Into::into).collect(),
        }
    }
}
