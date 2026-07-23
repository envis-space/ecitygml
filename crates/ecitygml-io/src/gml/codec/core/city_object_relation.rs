use crate::Error;
use crate::gml::codec::generics::GmlGenericAttributeProperty;
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::core::CityObjectRelation;
use ecitygml_core::model::generics::GenericAttributeKind;
use egml::io::codec::base::GmlReference;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{Formatting, XmlNode, XmlNodeContent, XmlNodeParts, serialize_inner};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_city_object_relation(xml_document: &[u8]) -> Result<CityObjectRelation, Error> {
    let parsed: GmlCityObjectRelation = de::from_reader(xml_document)?;
    parsed.try_into()
}

pub fn serialize_city_object_relation(
    relation: &CityObjectRelation,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(raw) = serialize_inner(GmlCityObjectRelation::from(relation), formatting)? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::CityObjectRelation.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCityObjectRelation {
    #[serde(rename = "relationType")]
    pub relation_type: GmlCode,

    #[serde(rename = "relatedTo")]
    pub related_to: GmlReference,

    #[serde(rename = "genericAttribute", default)]
    pub generic_attributes: Vec<GmlGenericAttributeProperty>,
}

impl TryFrom<GmlCityObjectRelation> for CityObjectRelation {
    type Error = Error;

    fn try_from(item: GmlCityObjectRelation) -> Result<Self, Self::Error> {
        let generic_attributes: Result<Vec<GenericAttributeKind>, _> = item
            .generic_attributes
            .into_iter()
            .map(|p| GenericAttributeKind::try_from(p.content))
            .collect();

        let mut relation = CityObjectRelation::new(
            Code::from(item.relation_type).into(),
            item.related_to.try_into()?,
        );
        relation.set_generic_attributes(generic_attributes?);
        Ok(relation)
    }
}

impl From<&CityObjectRelation> for GmlCityObjectRelation {
    fn from(item: &CityObjectRelation) -> Self {
        Self {
            relation_type: item.relation_type().code().into(),
            related_to: item.relation_to().into(),
            generic_attributes: item
                .generic_attributes()
                .iter()
                .map(|x| GmlGenericAttributeProperty { content: x.into() })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml::model::base::HasAssociationAttributes;
    use egml::model::xlink::HRef;
    use quick_xml::de;

    fn full_xml() -> &'static str {
        "<CityObjectRelation>\
          <genericAttribute>\
            <StringAttribute>\
              <name>identifier_roadObjectId</name>\
              <value>4970002</value>\
            </StringAttribute>\
          </genericAttribute>\
          <genericAttribute>\
            <StringAttribute>\
              <name>identifier_roadObjectName</name>\
              <value>tree</value>\
            </StringAttribute>\
          </genericAttribute>\
          <genericAttribute>\
            <StringAttribute>\
              <name>identifier_roadId</name>\
              <value>1970000</value>\
            </StringAttribute>\
          </genericAttribute>\
          <relationType>relatedVegetation</relationType>\
          <relatedTo href=\"#UUID_e8b543d5-43f3-3b07-94ee-c8e76419b29f\"/>\
        </CityObjectRelation>"
    }

    #[test]
    fn test_deserialize_gml_city_object_relation() {
        let gml: GmlCityObjectRelation =
            de::from_reader(full_xml().as_ref()).expect("should parse");

        assert_eq!(gml.relation_type.value, "relatedVegetation");
        assert_eq!(
            gml.related_to.association.href,
            Some("#UUID_e8b543d5-43f3-3b07-94ee-c8e76419b29f".to_string())
        );
        assert_eq!(gml.generic_attributes.len(), 3);
    }

    #[test]
    fn test_convert_to_city_object_relation() {
        let gml: GmlCityObjectRelation =
            de::from_reader(full_xml().as_ref()).expect("should parse");
        let relation: CityObjectRelation = gml.try_into().expect("should convert");

        assert_eq!(relation.relation_type().value(), "relatedVegetation");
        assert_eq!(
            relation.relation_to().href().expect("should have href"),
            &HRef::from("#UUID_e8b543d5-43f3-3b07-94ee-c8e76419b29f")
        );
        assert_eq!(relation.generic_attributes().len(), 3);
    }

    #[test]
    fn test_generic_attribute_names() {
        let gml: GmlCityObjectRelation =
            de::from_reader(full_xml().as_ref()).expect("should parse");
        let relation: CityObjectRelation = gml.try_into().expect("should convert");

        let names: Vec<&str> = relation
            .generic_attributes()
            .iter()
            .map(|a| a.name())
            .collect();

        assert!(names.contains(&"identifier_roadObjectId"));
        assert!(names.contains(&"identifier_roadObjectName"));
        assert!(names.contains(&"identifier_roadId"));
    }

    #[test]
    fn test_deserialize_without_generic_attributes() {
        let xml = "<CityObjectRelation>\
          <relationType>adjecent</relationType>\
          <relatedTo href=\"#UUID_abc123\"/>\
        </CityObjectRelation>";

        let gml: GmlCityObjectRelation = de::from_reader(xml.as_ref()).expect("should parse");
        let relation: CityObjectRelation = gml.try_into().expect("should convert");

        assert_eq!(relation.relation_type().value(), "adjecent");
        assert_eq!(
            relation.relation_to().href().expect("should have an href"),
            &HRef::from("#UUID_abc123")
        );
        assert!(relation.generic_attributes().is_empty());
    }

    #[test]
    fn test_missing_relation_type_returns_error() {
        let xml = "<CityObjectRelation>\
          <relatedTo href=\"#UUID_abc123\"/>\
        </CityObjectRelation>";

        let result: Result<GmlCityObjectRelation, _> = de::from_reader(xml.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip() {
        let gml: GmlCityObjectRelation =
            de::from_reader(full_xml().as_ref()).expect("should parse");
        let relation: CityObjectRelation = gml.try_into().expect("should convert");
        let gml_back = GmlCityObjectRelation::from(&relation);

        assert_eq!(gml_back.relation_type.value, "relatedVegetation");
        assert_eq!(
            gml_back
                .related_to
                .association
                .href
                .expect("should have an href")
                .as_str(),
            "#UUID_e8b543d5-43f3-3b07-94ee-c8e76419b29f"
        );
        assert_eq!(gml_back.generic_attributes.len(), 3);
    }
}
