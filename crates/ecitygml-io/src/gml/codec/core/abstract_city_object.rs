use crate::Error;
use crate::gml::codec::core::abstract_feature_with_lifespan::{
    GmlAbstractFeatureWithLifespan, deserialize_abstract_feature_with_lifespan,
};
use crate::gml::codec::core::external_reference_property::GmlExternalReferenceProperty;
use crate::gml::codec::core::relative_to_terrain::GmlRelativeToTerrain;
use crate::gml::codec::core::relative_to_water::GmlRelativeToWater;
use crate::gml::codec::generics::{GmlGenericAttributeKind, GmlGenericAttributeProperty};
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::{
    AbstractCityObject, AsAbstractCityObjectMut, AsAbstractFeatureWithLifespan,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_city_object(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractCityObject, Error> {
    let (abstract_feature_with_lifespan_result, parsed_result) = rayon::join(
        || deserialize_abstract_feature_with_lifespan(xml_document, spans),
        || de::from_reader::<_, GmlAbstractCityObject>(xml_document).map_err(Error::from),
    );

    let abstract_feature_with_lifespan = abstract_feature_with_lifespan_result?;
    let parsed = parsed_result?;

    let convert = |items: Vec<_>| {
        items
            .into_iter()
            .map(|x: GmlExternalReferenceProperty| x.content.try_into())
            .collect::<Result<Vec<_>, _>>()
    };
    let convert_generic = |items: Vec<_>| {
        items
            .into_iter()
            .map(|x: GmlGenericAttributeProperty| x.content.try_into())
            .collect::<Result<Vec<_>, _>>()
    };

    let mut abstract_city_object = AbstractCityObject::new(abstract_feature_with_lifespan);
    abstract_city_object.set_relative_to_terrain(parsed.relative_to_terrain.map(|x| x.into()));
    abstract_city_object.set_relative_to_water(parsed.relative_to_water.map(|x| x.into()));
    abstract_city_object.set_external_references(convert(parsed.external_references)?);
    abstract_city_object.set_generic_attributes(convert_generic(parsed.generic_attributes)?);

    Ok(abstract_city_object)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractCityObject {
    #[serde(flatten, skip_deserializing)]
    pub abstract_feature_with_lifespan: GmlAbstractFeatureWithLifespan,

    #[serde(
        rename = "externalReference",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_references: Vec<GmlExternalReferenceProperty>,

    #[serde(rename = "relativeToTerrain")]
    pub relative_to_terrain: Option<GmlRelativeToTerrain>,
    #[serde(rename = "relativeToWater")]
    pub relative_to_water: Option<GmlRelativeToWater>,

    #[serde(
        rename = "genericAttribute",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub generic_attributes: Vec<GmlGenericAttributeProperty>,
}

impl From<&AbstractCityObject> for GmlAbstractCityObject {
    fn from(city_object: &AbstractCityObject) -> Self {
        let external_references = city_object
            .external_references
            .iter()
            .map(|x| GmlExternalReferenceProperty { content: x.into() })
            .collect();

        let generic_attribute = city_object
            .generic_attributes
            .iter()
            .map(|a| GmlGenericAttributeProperty {
                content: GmlGenericAttributeKind::from(a),
            })
            .collect();

        Self {
            abstract_feature_with_lifespan: city_object.abstract_feature_with_lifespan().into(),
            external_references,
            relative_to_terrain: city_object.relative_to_terrain.map(|x| x.into()),
            relative_to_water: city_object.relative_to_water.map(|x| x.into()),
            generic_attributes: generic_attribute,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::util::extract_xml_element_spans;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_with_name_and_generic_attributes() {
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let city_object =
            deserialize_abstract_city_object(xml_document, &spans).expect("should work");

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
    fn test_deserialize_with_multiple_generic_attribute_types() {
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let city_object =
            deserialize_abstract_city_object(xml_document, &spans).expect("should work");

        assert_eq!(
            city_object.id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.name().is_empty());
        assert_eq!(city_object.generic_attributes.len(), 3);
    }

    #[test]
    fn test_deserialize_with_measure_generic_attribute() {
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let city_object =
            deserialize_abstract_city_object(xml_document, &spans).expect("should work");

        assert_eq!(
            city_object.id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.name().is_empty());
        assert_eq!(city_object.generic_attributes.len(), 2);
    }
}
