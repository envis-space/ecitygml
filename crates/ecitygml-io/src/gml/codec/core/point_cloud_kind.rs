use crate::gml::codec::point_cloud::{deserialize_point_cloud, serialize_point_cloud};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::{Error, Formatting};
use ecitygml_core::model::core::PointCloudKind;

pub fn deserialize_point_cloud_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<PointCloudKind>, Error> {
    if let Some(span) = spans.first(XmlElement::PointCloud) {
        let point_cloud = deserialize_point_cloud(&xml_document[span.start..span.end])?;
        return Ok(Some(point_cloud.into()));
    }

    Ok(None)
}

pub fn serialize_point_cloud_kind(
    point_cloud_kind: &PointCloudKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match point_cloud_kind {
        PointCloudKind::PointCloud(x) => serialize_point_cloud(x, formatting),
    }
}
