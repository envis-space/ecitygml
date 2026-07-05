use crate::Error;

use crate::gml::util::{XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use crate::gml::write::Formatting;

use crate::gml::codec::core::{deserialize_abstract_feature, serialize_abstract_feature};
use ecitygml_core::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, AsAbstractSurfaceDataMut,
};
use ecitygml_core::model::core::AsAbstractFeature;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_surface_data(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractSurfaceData, Error> {
    let (abstract_feature_result, gml_result) = rayon::join(
        || deserialize_abstract_feature(xml_document, spans),
        || de::from_reader::<_, GmlAbstractSurfaceData>(xml_document).map_err(Error::from),
    );
    let abstract_feature = abstract_feature_result?;
    let gml = gml_result?;

    let mut abstract_surface_data = AbstractSurfaceData::from_abstract_feature(abstract_feature);
    abstract_surface_data.set_is_front(gml.is_front);

    Ok(abstract_surface_data)
}

pub fn serialize_abstract_surface_data(
    abstract_surface_data: &AbstractSurfaceData,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts =
        serialize_abstract_feature(abstract_surface_data.abstract_feature(), formatting)?;

    if let Some(raw) = serialize_inner(
        GmlAbstractSurfaceData::from(abstract_surface_data),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSurfaceData {
    #[serde(
        rename(serialize = "app:isFront", deserialize = "isFront"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_front: Option<bool>,
}

impl From<&AbstractSurfaceData> for GmlAbstractSurfaceData {
    fn from(item: &AbstractSurfaceData) -> Self {
        Self {
            is_front: item.is_front().copied(),
        }
    }
}
