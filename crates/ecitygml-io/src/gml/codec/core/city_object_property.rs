use crate::Error;
use crate::gml::codec::core::city_object_kind::{
    deserialize_city_object_kind, serialize_city_object_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::CityObjectProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_city_object_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<CityObjectProperty, Error> {
    let gml_city_object_property: GmlCityObjectProperty = de::from_reader(xml_document)?;
    let mut city_object_property: CityObjectProperty = gml_city_object_property.into();

    city_object_property.object = deserialize_city_object_kind(xml_document, spans)?;

    Ok(city_object_property)
}

pub fn serialize_city_object_member_property(
    city_object_property: &CityObjectProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    if let Some(href) = &city_object_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &city_object_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_city_object_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(XmlElement::CityObjectMemberProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCityObjectProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlCityObjectProperty> for CityObjectProperty {
    fn from(item: GmlCityObjectProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}

/*impl From<&CityObjectProperty> for GmlCityObjectProperty {
    fn from(item: &CityObjectProperty) -> Self {
        Self {
            object: item.object.as_ref().map(Into::into),
            href: item.href.clone(),
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_basic_href_boundary() {
        use crate::gml::util::extract_xml_element_spans;
        let xml_document =
            b"<core:cityObjectMember xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2\" />";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let filling_surface_property =
            deserialize_city_object_property(xml_document, &spans).expect("should work");

        assert_eq!(
            filling_surface_property.href.as_deref(),
            Some("#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2")
        );
    }
}
