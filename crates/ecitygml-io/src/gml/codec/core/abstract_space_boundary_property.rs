use crate::Error;
use crate::gml::codec::core::abstract_space_boundary_kind::{
    deserialize_abstract_space_boundary_kind, serialize_abstract_space_boundary_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractSpaceBoundaryProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_space_boundary_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractSpaceBoundaryProperty, Error> {
    let parsed: GmlSpaceBoundaryProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_space_boundary_kind(xml_document, spans)?;

    Ok(AbstractSpaceBoundaryProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_space_boundary_property(
    abstract_space_boundary_property: &AbstractSpaceBoundaryProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_space_boundary_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_space_boundary_property.ownership(),
    ));

    if let Some(object) = abstract_space_boundary_property.object() {
        parts.content.push(XmlNodeContent::Child(
            serialize_abstract_space_boundary_kind(object, formatting)?,
        ));
    }

    Ok(XmlNode::new(CityGmlElement::BoundaryProperty.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSpaceBoundaryProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::io::util::extract_xml_element_spans;
    use egml::model::base::{HasAssociationAttributes, Id};
    use egml::model::xlink::HRef;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_href_boundary() {
        use egml::io::util::extract_xml_element_spans;
        let xml_document =
            b"<boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2\" />";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space_boundary_property =
            deserialize_abstract_space_boundary_property(xml_document, &spans)
                .expect("should work");

        assert_eq!(
            abstract_space_boundary_property
                .href()
                .expect("should exist"),
            &HRef::from("#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2")
        );
    }

    #[test]
    fn test_deserialize_empty_closure_surface() {
        let xml_document =
            b"<boundary><ClosureSurface gml:id=\"fme-gen-f460209b-63b5-4ff8-92d5-cec932987265\"/></boundary>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space_boundary_property =
            deserialize_abstract_space_boundary_property(xml_document, &spans)
                .expect("should work");

        assert_eq!(
            abstract_space_boundary_property
                .object()
                .unwrap()
                .feature_id(),
            &Id::try_from("fme-gen-f460209b-63b5-4ff8-92d5-cec932987265").expect("should work")
        );
    }
}
