use crate::Error;
use crate::gml::codec::relief::relief_component_kind::deserialize_relief_component_kind;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::relief::ReliefComponentProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_relief_component_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<ReliefComponentProperty, Error> {
    let gml_relief_component_property: GmlReliefComponentProperty = de::from_reader(xml_document)?;
    let mut relief_component_property: ReliefComponentProperty =
        gml_relief_component_property.into();

    relief_component_property.object = deserialize_relief_component_kind(xml_document, spans)?;

    Ok(relief_component_property)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlReliefComponentProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlReliefComponentProperty> for ReliefComponentProperty {
    fn from(item: GmlReliefComponentProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}
