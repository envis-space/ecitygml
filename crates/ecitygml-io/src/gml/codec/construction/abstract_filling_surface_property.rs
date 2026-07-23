use crate::Error;
use crate::gml::codec::construction::abstract_filling_surface_kind::{
    deserialize_abstract_filling_surface_kind, serialize_abstract_filling_surface_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::construction::AbstractFillingSurfaceProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_filling_surface_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractFillingSurfaceProperty, Error> {
    let parsed: GmlFillingSurfaceProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_filling_surface_kind(xml_document, spans)?;

    Ok(AbstractFillingSurfaceProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_filling_surface_property(
    abstract_filling_surface_property: &AbstractFillingSurfaceProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_filling_surface_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_filling_surface_property.ownership(),
    ));

    if let Some(object) = abstract_filling_surface_property.object() {
        parts.content.push(XmlNodeContent::Child(
            serialize_abstract_filling_surface_kind(object, formatting)?,
        ));
    }

    Ok(XmlNode::new(
        CityGmlElement::AbstractFillingSurfaceProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlFillingSurfaceProperty {
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
    use egml::model::base::{HasAssociationAttributes, Id};
    use egml::model::xlink::HRef;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_href_boundary() {
        use egml::io::util::extract_xml_element_spans;
        let xml_document =
            b"<con:fillingSurface xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2\" />";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_filling_surface_property =
            deserialize_abstract_filling_surface_property(xml_document, &spans)
                .expect("should work");

        assert_eq!(
            abstract_filling_surface_property
                .href()
                .expect("should exist"),
            &HRef::from("#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2")
        );
    }
}
