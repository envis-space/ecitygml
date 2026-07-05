use crate::gml::codec::core::{deserialize_abstract_point_cloud, serialize_abstract_point_cloud};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::{Error, Formatting};
use ecitygml_core::model::core::AsAbstractPointCloud;
use ecitygml_core::model::point_cloud::PointCloud;
use egml::io::GmlCode;
use egml::io::aggregates::GmlMultiPointProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_point_cloud(xml_document: &[u8]) -> Result<PointCloud, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_point_cloud_result, parsed_result) = rayon::join(
        || deserialize_abstract_point_cloud(xml_document, &spans),
        || de::from_reader::<_, GmlPointCloud>(xml_document).map_err(Error::from),
    );
    let abstract_point_cloud = abstract_point_cloud_result?;
    let parsed = parsed_result?;

    let mut point_cloud = PointCloud::from_abstract_point_cloud(abstract_point_cloud);
    point_cloud.set_mime_type(parsed.mime_type.map(Into::into));
    point_cloud.set_point_file(parsed.point_file);
    point_cloud.set_point_file_srs_name(parsed.point_file_srs_name);
    point_cloud.set_points(parsed.points.map(|x| x.try_into()).transpose()?);

    Ok(point_cloud)
}

pub fn serialize_point_cloud(
    point_cloud: &PointCloud,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_point_cloud(point_cloud.abstract_point_cloud(), formatting)?;

    if let Some(raw) = serialize_inner(GmlPointCloud::from(point_cloud), formatting)? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::PointCloud, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointCloud {
    #[serde(
        rename(serialize = "pcl:mimeType", deserialize = "mimeType"),
        skip_serializing_if = "Option::is_none"
    )]
    pub mime_type: Option<GmlCode>,

    #[serde(
        rename(serialize = "pcl:pointFile", deserialize = "pointFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub point_file: Option<String>,

    #[serde(
        rename(serialize = "pcl:pointFileSrsName", deserialize = "pointFileSrsName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub point_file_srs_name: Option<String>,

    #[serde(
        rename(serialize = "pcl:points", deserialize = "points"),
        skip_serializing_if = "Option::is_none"
    )]
    pub points: Option<GmlMultiPointProperty>,
}

impl From<&PointCloud> for GmlPointCloud {
    fn from(item: &PointCloud) -> Self {
        Self {
            mime_type: item.mime_type().map(Into::into),
            point_file: item.point_file().map(Into::into),
            point_file_srs_name: item.point_file_srs_name().map(Into::into),
            points: item.points().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_point_cloud_id_and_file_metadata() {
        let xml_document = b"<pcl:PointCloud gml:id=\"test-point-cloud-001\">
  <pcl:pointFile>abc.las</pcl:pointFile>
  <pcl:pointFileSrsName>EPSG:25832</pcl:pointFileSrsName>
</pcl:PointCloud>";

        let point_cloud = deserialize_point_cloud(xml_document).expect("should work");

        assert_eq!(
            point_cloud.id(),
            &Id::try_from("test-point-cloud-001").expect("should work")
        );
        assert_eq!(point_cloud.point_file(), Some("abc.las"));
        assert_eq!(point_cloud.point_file_srs_name(), Some("EPSG:25832"));
        assert!(point_cloud.mime_type().is_none());
        assert!(point_cloud.points().is_none());
    }

    #[test]
    fn test_deserialize_point_cloud_point_member() {
        let xml_document = b"<pcl:PointCloud gml:id=\"test-point-cloud-002\">
  <pcl:points>
    <gml:MultiPoint>
      <gml:pointMember>
        <gml:Point>
          <gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos>
        </gml:Point>
      </gml:pointMember>
      <gml:pointMember>
        <gml:Point>
          <gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos>
        </gml:Point>
      </gml:pointMember>
    </gml:MultiPoint>
  </pcl:points>
</pcl:PointCloud>";

        let point_cloud = deserialize_point_cloud(xml_document).expect("should work");

        let multi_point = point_cloud
            .points()
            .expect("points should be present")
            .object
            .as_ref()
            .expect("MultiPoint object should be present");

        assert_eq!(multi_point.point_member().len(), 2);
        assert_eq!(
            multi_point.point_member()[0]
                .object
                .as_ref()
                .expect("point should be present")
                .pos()
                .x(),
            678267.6213956032
        );
        assert_eq!(
            multi_point.point_member()[1]
                .object
                .as_ref()
                .expect("point should be present")
                .pos()
                .x(),
            678289.06567932
        );
    }

    #[test]
    fn test_deserialize_point_cloud_point_members() {
        let xml_document = b"<pcl:PointCloud gml:id=\"test-point-cloud-003\">
  <pcl:points>
    <gml:MultiPoint>
      <gml:pointMembers>
        <gml:Point>
          <gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos>
        </gml:Point>
        <gml:Point>
          <gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos>
        </gml:Point>
      </gml:pointMembers>
    </gml:MultiPoint>
  </pcl:points>
</pcl:PointCloud>";

        let point_cloud = deserialize_point_cloud(xml_document).expect("should work");

        let multi_point = point_cloud
            .points()
            .expect("points should be present")
            .object
            .as_ref()
            .expect("MultiPoint object should be present");

        assert!(multi_point.point_member().is_empty());

        let point_members = multi_point
            .point_members()
            .expect("pointMembers should be present");
        assert_eq!(point_members.objects.len(), 2);
        assert_eq!(point_members.objects[0].pos().x(), 678267.6213956032);
        assert_eq!(point_members.objects[1].pos().x(), 678289.06567932);
    }
}
