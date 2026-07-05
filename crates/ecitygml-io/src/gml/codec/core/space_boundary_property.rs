use crate::Error;
use crate::gml::codec::core::space_boundary_kind::{
    deserialize_space_boundary_kind, serialize_space_boundary_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::SpaceBoundaryProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_space_boundary_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<SpaceBoundaryProperty, Error> {
    let gml_space_boundary_property: GmlSpaceBoundaryProperty = de::from_reader(xml_document)?;
    let mut space_boundary_property: SpaceBoundaryProperty = gml_space_boundary_property.into();

    space_boundary_property.object = deserialize_space_boundary_kind(xml_document, spans)?;

    Ok(space_boundary_property)
}

pub fn serialize_space_boundary_property(
    space_boundary_property: &SpaceBoundaryProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    if let Some(href) = &space_boundary_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }

    if let Some(object) = &space_boundary_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_space_boundary_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(XmlElement::BoundaryProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSpaceBoundaryProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlSpaceBoundaryProperty> for SpaceBoundaryProperty {
    fn from(item: GmlSpaceBoundaryProperty) -> Self {
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
    use crate::gml::util::extract_xml_element_spans;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_href_boundary() {
        use crate::gml::util::extract_xml_element_spans;
        let xml_document =
            b"<boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2\" />";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let space_boundary_property =
            deserialize_space_boundary_property(xml_document, &spans).expect("should work");

        assert_eq!(
            space_boundary_property.href.as_deref(),
            Some("#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2")
        );
    }

    #[test]
    fn test_deserialize_empty_closure_surface() {
        let xml_document =
            b"<boundary><ClosureSurface gml:id=\"fme-gen-f460209b-63b5-4ff8-92d5-cec932987265\"/></boundary>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let space_boundary_property =
            deserialize_space_boundary_property(xml_document, &spans).expect("should work");

        assert_eq!(
            space_boundary_property.object.unwrap().id(),
            &Id::try_from("fme-gen-f460209b-63b5-4ff8-92d5-cec932987265").expect("should work")
        );
    }
}
