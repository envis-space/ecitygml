use crate::Error;
use crate::gml::codec::transportation::deserialize_traffic_space;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::transportation::TrafficSpaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_traffic_space_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<TrafficSpaceProperty, Error> {
    let gml_traffic_space_property: GmlTrafficSpaceProperty = de::from_reader(xml_document)?;
    let mut traffic_space_property: TrafficSpaceProperty = gml_traffic_space_property.into();

    if let Some(span) = spans.first(XmlElement::TrafficSpace) {
        traffic_space_property.object = Some(deserialize_traffic_space(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(traffic_space_property)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficSpaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlTrafficSpaceProperty> for TrafficSpaceProperty {
    fn from(item: GmlTrafficSpaceProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
