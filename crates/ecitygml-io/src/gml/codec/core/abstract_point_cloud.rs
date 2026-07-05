use crate::gml::codec::core::{deserialize_abstract_feature, serialize_abstract_feature};
use crate::gml::util::{XmlElementSpans, XmlNodeParts};
use crate::{Error, Formatting};
use ecitygml_core::model::core::{AbstractPointCloud, AsAbstractFeature};

pub fn deserialize_abstract_point_cloud(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractPointCloud, Error> {
    let abstract_feature = deserialize_abstract_feature(xml_document, spans)?;
    let abstract_point_cloud = AbstractPointCloud::from_abstract_feature(abstract_feature);

    Ok(abstract_point_cloud)
}

pub fn serialize_abstract_point_cloud(
    abstract_point_cloud: &AbstractPointCloud,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let xml_node_parts =
        serialize_abstract_feature(abstract_point_cloud.abstract_feature(), formatting)?;

    Ok(xml_node_parts)
}
