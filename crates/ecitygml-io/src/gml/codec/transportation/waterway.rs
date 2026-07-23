use crate::Error;
use crate::gml::codec::transportation::abstract_transportation_space::{
    deserialize_abstract_transportation_space, serialize_abstract_transportation_space,
};
use crate::gml::codec::transportation::intersection_property::{
    deserialize_intersection_property, serialize_intersection_property,
};
use crate::gml::codec::transportation::section_property::{
    deserialize_section_property, serialize_section_property,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::transportation::values::{
    WaterwayClassValue, WaterwayFunctionValue, WaterwayUsageValue,
};
use ecitygml_core::model::transportation::{AsAbstractTransportationSpace, Waterway};
use egml::io::codec::basic::GmlCode;
use egml::io::util::collect_children;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

pub fn deserialize_waterway(xml_document: &[u8]) -> Result<Waterway, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut abstract_transportation_space_result = None;
    let mut parsed_result = None;
    let mut sections_result = None;
    let mut intersections_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_transportation_space_result = Some(deserialize_abstract_transportation_space(
                xml_document,
                &spans,
            ))
        });
        s.spawn(|_| {
            parsed_result = Some(
                quick_xml::de::from_reader::<_, GmlWaterway>(xml_document).map_err(Error::from),
            );
        });
        s.spawn(|_| {
            sections_result = Some(collect_children(
                xml_document,
                &spans,
                CityGmlElement::SectionProperty.into(),
                deserialize_section_property,
            ));
        });
        s.spawn(|_| {
            intersections_result = Some(collect_children(
                xml_document,
                &spans,
                CityGmlElement::IntersectionProperty.into(),
                deserialize_intersection_property,
            ));
        });
    });

    let abstract_transportation_space = abstract_transportation_space_result
        .expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let sections = sections_result.expect("rayon::scope guarantees all spawns complete")?;
    let intersections =
        intersections_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut waterway = Waterway::from_abstract_transportation_space(abstract_transportation_space);
    waterway.set_class_opt(parsed.class.map(Code::from).map(WaterwayClassValue::from));
    waterway.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(WaterwayFunctionValue::from)
            .collect(),
    );
    waterway.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(WaterwayUsageValue::from)
            .collect(),
    );
    waterway.set_sections(sections);
    waterway.set_intersections(intersections);

    Ok(waterway)
}

pub fn serialize_waterway(waterway: &Waterway, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_transportation_space(
        waterway.abstract_transportation_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(GmlWaterway::from(waterway), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for section_property in waterway.sections() {
        let node = serialize_section_property(section_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    for intersection_property in waterway.intersections() {
        let node = serialize_intersection_property(intersection_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    Ok(XmlNode::new(
        CityGmlElement::Waterway.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlWaterway {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "tran:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "tran:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&Waterway> for GmlWaterway {
    fn from(item: &Waterway) -> Self {
        Self {
            class: item.class().map(WaterwayClassValue::code).map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(WaterwayFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(WaterwayUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace};
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_waterway() {
        let xml_document = b"
    <tran:Waterway gml:id=\"UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f\">
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>identifier_new_section</gen:name>
          <gen:value>abc</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <tran:section>
        <tran:Section gml:id=\"UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29\">
        </tran:Section>
      </tran:section>
    </tran:Waterway>";

        let waterway = deserialize_waterway(xml_document).expect("should work");

        assert_eq!(
            waterway.feature_id(),
            &Id::try_from("UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f").expect("should work")
        );

        assert!(waterway.lod2_multi_surface().is_none());
        assert_eq!(waterway.generic_attributes().len(), 1);
        assert!(waterway.intersections().is_empty());
        assert_eq!(waterway.sections().len(), 1);
        let traffic_space = waterway.sections().first().unwrap().object().unwrap();
        assert_eq!(
            traffic_space.feature_id(),
            &Id::try_from("UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29").expect("should work")
        );
    }
}
