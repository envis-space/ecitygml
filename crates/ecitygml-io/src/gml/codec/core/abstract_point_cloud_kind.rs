use crate::Error;
use crate::gml::codec::point_cloud::{deserialize_point_cloud, serialize_point_cloud};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractPointCloudKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_point_cloud_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractPointCloudKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::PointCloud.into()) {
        let point_cloud = deserialize_point_cloud(&xml_document[span.start..span.end])?;
        return Ok(Some(point_cloud.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_point_cloud_kind(
    abstract_point_cloud_kind: &AbstractPointCloudKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_point_cloud_kind {
        AbstractPointCloudKind::PointCloud(x) => serialize_point_cloud(x, formatting),
    }
}
