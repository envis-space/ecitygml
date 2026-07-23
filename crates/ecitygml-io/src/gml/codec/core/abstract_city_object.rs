use crate::Error;
use crate::gml::codec::core::abstract_appearance_property::{
    deserialize_abstract_appearance_property, serialize_abstract_appearance_property,
};
use crate::gml::codec::core::abstract_feature_with_lifespan::deserialize_abstract_feature_with_lifespan;
use crate::gml::codec::core::city_object_relation_property::{
    deserialize_city_object_relation_property, serialize_city_object_relation_property,
};
use crate::gml::codec::core::enums::GmlRelativeToTerrain;
use crate::gml::codec::core::enums::GmlRelativeToWater;
use crate::gml::codec::core::external_reference_property::GmlExternalReferenceProperty;
use crate::gml::codec::core::serialize_abstract_feature_with_lifespan;
use crate::gml::codec::generics::{GmlGenericAttributeKind, GmlGenericAttributeProperty};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::{
    AbstractCityObject, AsAbstractCityObject, AsAbstractCityObjectMut,
    AsAbstractFeatureWithLifespan,
};
use egml::io::util::collect_children;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_city_object(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractCityObject, Error> {
    let mut abstract_feature_with_lifespan_result = None;
    let mut parsed_result = None;
    let mut appearances_result = None;
    let mut related_to_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_feature_with_lifespan_result = Some(
                deserialize_abstract_feature_with_lifespan(xml_document, spans),
            );
        });
        s.spawn(|_| {
            parsed_result = Some(
                quick_xml::de::from_reader::<_, GmlAbstractCityObject>(xml_document)
                    .map_err(Error::from),
            );
        });
        s.spawn(|_| {
            appearances_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::AbstractAppearanceProperty.into(),
                deserialize_abstract_appearance_property,
            ));
        });
        s.spawn(|_| {
            related_to_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::RelatedToProperty.into(),
                deserialize_city_object_relation_property,
            ));
        });
    });

    let abstract_feature_with_lifespan = abstract_feature_with_lifespan_result
        .expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let appearances = appearances_result.expect("rayon::scope guarantees all spawns complete")?;
    let related_to = related_to_result.expect("rayon::scope guarantees all spawns complete")?;

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

    let mut abstract_city_object =
        AbstractCityObject::from_abstract_feature_with_lifespan(abstract_feature_with_lifespan);
    abstract_city_object.set_relative_to_terrain(parsed.relative_to_terrain.map(Into::into));
    abstract_city_object.set_relative_to_water(parsed.relative_to_water.map(Into::into));
    abstract_city_object.set_external_references(convert(parsed.external_references)?);
    abstract_city_object.set_generic_attributes(convert_generic(parsed.generic_attributes)?);

    abstract_city_object.set_appearances(appearances);
    abstract_city_object.set_related_to(related_to);

    Ok(abstract_city_object)
}

pub fn serialize_abstract_city_object(
    abstract_city_object: &AbstractCityObject,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_feature_with_lifespan(
        abstract_city_object.abstract_feature_with_lifespan(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractCityObject::from(abstract_city_object),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for prop in abstract_city_object.appearances() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_abstract_appearance_property(prop, formatting)?,
        ));
    }

    for prop in abstract_city_object.related_to() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_city_object_relation_property(prop, formatting)?,
        ));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractCityObject {
    #[serde(rename = "externalReference", default)]
    pub external_references: Vec<GmlExternalReferenceProperty>,

    #[serde(rename = "relativeToTerrain", skip_serializing_if = "Option::is_none")]
    pub relative_to_terrain: Option<GmlRelativeToTerrain>,
    #[serde(rename = "relativeToWater", skip_serializing_if = "Option::is_none")]
    pub relative_to_water: Option<GmlRelativeToWater>,

    #[serde(rename = "genericAttribute", default)]
    pub generic_attributes: Vec<GmlGenericAttributeProperty>,
}

impl From<&AbstractCityObject> for GmlAbstractCityObject {
    fn from(city_object: &AbstractCityObject) -> Self {
        let external_references = city_object
            .external_references()
            .iter()
            .map(|x| GmlExternalReferenceProperty { content: x.into() })
            .collect();

        let generic_attribute = city_object
            .generic_attributes()
            .iter()
            .map(|a| GmlGenericAttributeProperty {
                content: GmlGenericAttributeKind::from(a),
            })
            .collect();

        Self {
            external_references,
            relative_to_terrain: city_object.relative_to_terrain().map(Into::into),
            relative_to_water: city_object.relative_to_water().map(Into::into),
            generic_attributes: generic_attribute,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::io::util::extract_xml_element_spans;
    use egml::model::base::{AsAbstractGml, HasAssociationAttributes, Id};
    use egml::model::basic_types::Code;
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
            city_object.feature_id(),
            &Id::try_from("test-id-123").expect("should work")
        );
        assert_eq!(
            city_object.names().first().expect("name should be present"),
            &Code::new("West wall")
        );
        assert_eq!(city_object.generic_attributes().len(), 2);
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
            city_object.feature_id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.names().is_empty());
        assert_eq!(city_object.generic_attributes().len(), 3);
    }

    #[test]
    fn test_deserialize_with_related_to() {
        let xml_document = b"
    <tran:TrafficSpace gml:id=\"UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3\">
      <relatedTo>
        <CityObjectRelation>
          <genericAttribute>
            <gen:StringAttribute>
              <gen:name>identifier_roadObjectId</gen:name>
              <gen:value>4970008</gen:value>
            </gen:StringAttribute>
          </genericAttribute>
          <relationType>relatedFurniture</relationType>
          <relatedTo xlink:href=\"#UUID_aca562a7-20d4-3159-b121-e50cc6eb0ade\"/>
        </CityObjectRelation>
      </relatedTo>
    </tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let city_object =
            deserialize_abstract_city_object(xml_document, &spans).expect("should work");

        assert_eq!(city_object.related_to().len(), 1);

        let relation = city_object.related_to()[0]
            .object()
            .expect("relation should be inline");

        assert_eq!(relation.relation_type().value(), "relatedFurniture");
        assert_eq!(
            relation.relation_to().href().expect("should have href"),
            &egml::model::xlink::HRef::from("#UUID_aca562a7-20d4-3159-b121-e50cc6eb0ade")
        );
        assert_eq!(relation.generic_attributes().len(), 1);
    }

    #[test]
    fn test_serialize_round_trip_related_to() {
        let xml_document = b"
    <tran:TrafficSpace gml:id=\"UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3\">
      <relatedTo>
        <CityObjectRelation>
          <relationType>relatedFurniture</relationType>
          <relatedTo xlink:href=\"#UUID_aca562a7-20d4-3159-b121-e50cc6eb0ade\"/>
        </CityObjectRelation>
      </relatedTo>
    </tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let city_object =
            deserialize_abstract_city_object(xml_document, &spans).expect("should work");

        let xml_node_parts =
            serialize_abstract_city_object(&city_object, Formatting::Compact).expect("should work");

        let serialized = xml_node_parts
            .content
            .iter()
            .find_map(|content| match content {
                XmlNodeContent::Child(node) if node.name == "relatedTo" => Some(node),
                _ => None,
            })
            .expect("relatedTo child should be present");

        assert_eq!(serialized.name, "relatedTo");
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
            city_object.feature_id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.names().is_empty());
        assert_eq!(city_object.generic_attributes().len(), 2);
    }
}
