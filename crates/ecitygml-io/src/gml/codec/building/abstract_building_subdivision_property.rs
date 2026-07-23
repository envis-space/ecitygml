use crate::Error;
use crate::gml::codec::building::abstract_building_subdivision_kind::{
    deserialize_abstract_building_subdivision_kind, serialize_abstract_building_subdivision_kind,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::AbstractBuildingSubdivisionProperty;
use egml::io::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use egml::io::util::{Formatting, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_building_subdivision_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractBuildingSubdivisionProperty, Error> {
    let parsed: GmlBuildingSubdivisionProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_building_subdivision_kind(xml_document, spans)?;

    Ok(AbstractBuildingSubdivisionProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_building_subdivision_property(
    abstract_building_subdivision_property: &AbstractBuildingSubdivisionProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    parts.attributes.extend(serialize_association_attributes(
        abstract_building_subdivision_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_building_subdivision_property.ownership(),
    ));
    if let Some(object) = abstract_building_subdivision_property.object() {
        parts.content.push(XmlNodeContent::Child(
            serialize_abstract_building_subdivision_kind(object, formatting)?,
        ));
    }
    Ok(XmlNode::new(
        CityGmlElement::AbstractBuildingSubdivisionProperty.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingSubdivisionProperty {
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
    use egml::model::base::{AsAbstractGml, Id};
    use egml::model::basic_types::Code;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_href_boundary() {
        use egml::io::util::extract_xml_element_spans;
        let xml_document = b"<bldg:buildingSubdivision>
            <bldg:Storey>
              <gml:name>Second Floor</gml:name>
            </bldg:Storey>
          </bldg:buildingSubdivision>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_building_subdivision_property =
            deserialize_abstract_building_subdivision_property(xml_document, &spans)
                .expect("should work");
        let abstract_building_subdivision_kind =
            abstract_building_subdivision_property.object().unwrap();

        assert_eq!(
            abstract_building_subdivision_kind.names().first().unwrap(),
            &Code::new("Second Floor")
        );
    }
}
