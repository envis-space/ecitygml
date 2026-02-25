use crate::Error;
use crate::gml::parser::generics::GmlGenericAttributeProperty;
use ecitygml_core::model::core::{
    AbstractCityObject, AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut,
};
use ecitygml_core::model::generics::GenericAttributeKind;
use egml::model::base::Id;
use quick_xml::{DeError, de};
use serde::{Deserialize, Serialize};

pub fn parse_abstract_city_object(xml_document: &[u8]) -> Result<AbstractCityObject, Error> {
    let parsed_result: Result<GmlAbstractCityObject, DeError> = de::from_reader(xml_document);
    let mut parsed_gml = parsed_result.expect("parsing should work");

    if parsed_gml.id.is_none() {
        let id: Id = Id::from_hashed_bytes(xml_document);
        parsed_gml.id = Some(id.into());
    }
    let parsed_names = parsed_gml.name.clone();

    let mut abstract_city_object = AbstractCityObject::try_from(parsed_gml)?;
    abstract_city_object.set_name(parsed_names);
    Ok(abstract_city_object)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractCityObject {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "name", default)]
    pub name: Vec<String>,

    #[serde(rename = "genericAttribute", default)]
    pub generic_attribute: Vec<GmlGenericAttributeProperty>,
}

impl TryFrom<GmlAbstractCityObject> for AbstractCityObject {
    type Error = Error;

    fn try_from(gml: GmlAbstractCityObject) -> Result<Self, Self::Error> {
        let id = gml
            .id
            .as_ref()
            .map(Id::try_from)
            .expect("id must be present")?;

        let generic_attributes: Vec<GenericAttributeKind> = gml
            .generic_attribute
            .into_iter()
            .map(|x| x.content.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let abstract_feature = AbstractFeature::new(id);
        Ok(AbstractCityObject::new(
            abstract_feature,
            generic_attributes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_parse_city_object_basic() {
        let xml_document = b"
    <con:WallSurface gml:id=\"test-id-123\">
      <gml:name>West wall</gml:name>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>DatenquelleBodenhoehe</gen:name>
          <gen:value>1100</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>DatenquelleDachhoehe</gen:name>
          <gen:value>1000</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
    </con:WallSurface>";

        let city_object = parse_abstract_city_object(xml_document).expect("should work");

        assert_eq!(
            city_object.id(),
            &Id::try_from("test-id-123").expect("should work")
        );
        assert_eq!(
            city_object.name().first().expect("name should be present"),
            "West wall"
        );
        assert_eq!(city_object.generic_attributes.len(), 2);
    }

    #[test]
    fn test_parse_city_object_with_mixed_attributes_xml() {
        let xml_document = b"
    <bldg:Building gml:id=\"test-id\">
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>attribute_name_one</gen:name>
          <gen:value>1100</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <genericAttribute>
        <gen:IntAttribute>
          <gen:name>attribute_name_two</gen:name>
          <gen:value>1100</gen:value>
        </gen:IntAttribute>
      </genericAttribute>
       <genericAttribute>
        <gen:DoubleAttribute>
          <gen:name>attribute_name_three</gen:name>
          <gen:value>1100</gen:value>
        </gen:DoubleAttribute>
      </genericAttribute>
    </bldg:Building>";

        let city_object = parse_abstract_city_object(xml_document).expect("should work");

        assert_eq!(
            city_object.id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.name().is_empty());
        assert_eq!(city_object.generic_attributes.len(), 3);
    }

    #[test]
    fn test_parse_city_object_with_generic_measure_attribute() {
        let xml_document = b"
    <bldg:Building gml:id=\"test-id\">
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>attribute_name_one</gen:name>
          <gen:value>1100</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <genericAttribute>
        <gen:MeasureAttribute>
          <gen:name>GrossPlannedArea</gen:name>
          <gen:value uom=\"m2\">120.0</gen:value>
        </gen:MeasureAttribute>
      </genericAttribute>
    </bldg:Building>";

        let city_object = parse_abstract_city_object(xml_document).expect("should work");

        assert_eq!(
            city_object.id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.name().is_empty());
        assert_eq!(city_object.generic_attributes.len(), 2);
    }
}
