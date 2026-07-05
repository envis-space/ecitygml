use crate::Error;
use crate::gml::codec::transportation::{
    deserialize_auxiliary_traffic_space, serialize_auxiliary_traffic_space,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
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

pub fn serialize_auxiliary_traffic_space_property(
    property: &AuxiliaryTrafficSpaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_auxiliary_traffic_space(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(
        XmlElement::AuxiliaryTrafficSpaceProperty,
        parts,
    ))
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
