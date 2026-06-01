use crate::Error;
use crate::gml::codec::transportation::deserialize_auxiliary_traffic_space;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::transportation::AuxiliaryTrafficSpaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_auxiliary_traffic_space_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AuxiliaryTrafficSpaceProperty, Error> {
    let gml_auxiliary_traffic_space_property: GmlAuxiliaryTrafficSpaceProperty =
        de::from_reader(xml_document)?;
    let mut auxiliary_traffic_space_property: AuxiliaryTrafficSpaceProperty =
        gml_auxiliary_traffic_space_property.into();

    if let Some(span) = spans.first(XmlElement::AuxiliaryTrafficSpace) {
        auxiliary_traffic_space_property.object = Some(deserialize_auxiliary_traffic_space(
            &xml_document[span.start..span.end],
        )?);
    }

    Ok(auxiliary_traffic_space_property)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAuxiliaryTrafficSpaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlAuxiliaryTrafficSpaceProperty> for AuxiliaryTrafficSpaceProperty {
    fn from(item: GmlAuxiliaryTrafficSpaceProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
