use crate::Error;
use crate::gml::codec::construction::filling_surface_kind::{
    deserialize_filling_surface_kind, serialize_filling_surface_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::FillingSurfaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_filling_surface_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<FillingSurfaceProperty, Error> {
    let gml_filling_surface_property: GmlFillingSurfaceProperty = de::from_reader(xml_document)?;
    let mut filling_surface_property: FillingSurfaceProperty = gml_filling_surface_property.into();

    filling_surface_property.object = deserialize_filling_surface_kind(xml_document, spans)?;

    Ok(filling_surface_property)
}

pub fn serialize_filling_surface_property(
    filling_surface_property: &FillingSurfaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    if let Some(href) = &filling_surface_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &filling_surface_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_filling_surface_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(XmlElement::FillingSurfaceProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlFillingSurfaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlFillingSurfaceProperty> for FillingSurfaceProperty {
    fn from(item: GmlFillingSurfaceProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_href_boundary() {
        use crate::gml::util::extract_xml_element_spans;
        let xml_document =
            b"<con:fillingSurface xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2\" />";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let filling_surface_property =
            deserialize_filling_surface_property(xml_document, &spans).expect("should work");

        assert_eq!(
            filling_surface_property.href.as_deref(),
            Some("#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2")
        );
    }
}
