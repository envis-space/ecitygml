use crate::Error;
use crate::gml::codec::core::abstract_feature_with_lifespan::deserialize_abstract_feature_with_lifespan;
use crate::gml::codec::core::appearance_property::{
    deserialize_appearance_property, serialize_appearance_property,
};
use crate::gml::codec::core::enums::GmlRelativeToTerrain;
use crate::gml::codec::core::enums::GmlRelativeToWater;
use crate::gml::codec::core::external_reference_property::GmlExternalReferenceProperty;
use crate::gml::codec::core::serialize_abstract_feature_with_lifespan;
use crate::gml::codec::generics::{GmlGenericAttributeKind, GmlGenericAttributeProperty};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{
    XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_children, serialize_inner,
};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::{
    AbstractCityObject, AsAbstractCityObject, AsAbstractCityObjectMut,
    AsAbstractFeatureWithLifespan,
};
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_city_object(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractCityObject, Error> {
    let mut abstract_feature_with_lifespan_result = None;
    let mut parsed_result = None;
    let mut appearances_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_feature_with_lifespan_result = Some(
                deserialize_abstract_feature_with_lifespan(xml_document, spans),
            )
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
                XmlElement::AppearanceProperty,
                deserialize_appearance_property,
            ));
        });
    });

    let abstract_feature_with_lifespan = abstract_feature_with_lifespan_result
        .expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let appearances = appearances_result.expect("rayon::scope guarantees all spawns complete")?;

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
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_appearance_property(
                prop, formatting,
            )?));
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
            city_object.id(),
            &Id::try_from("test-id").expect("should work")
        );
        assert!(city_object.name().is_empty());
        assert_eq!(city_object.generic_attributes().len(), 3);
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
        assert_eq!(city_object.generic_attributes().len(), 2);
    }
}
