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
    RailwayClassValue, RailwayFunctionValue, RailwayUsageValue,
};
use ecitygml_core::model::transportation::{AsAbstractTransportationSpace, Railway};
use egml::io::codec::basic::GmlCode;
use egml::io::util::collect_children;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

pub fn deserialize_railway(xml_document: &[u8]) -> Result<Railway, Error> {
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
                quick_xml::de::from_reader::<_, GmlRailway>(xml_document).map_err(Error::from),
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

    let mut railway = Railway::from_abstract_transportation_space(abstract_transportation_space);
    railway.set_class_opt(parsed.class.map(Code::from).map(RailwayClassValue::from));
    railway.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(RailwayFunctionValue::from)
            .collect(),
    );
    railway.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(RailwayUsageValue::from)
            .collect(),
    );
    railway.set_sections(sections);
    railway.set_intersections(intersections);

    Ok(railway)
}

pub fn serialize_railway(railway: &Railway, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_transportation_space(
        railway.abstract_transportation_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(GmlRailway::from(railway), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for section_property in railway.sections() {
        let node = serialize_section_property(section_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    for intersection_property in railway.intersections() {
        let node = serialize_intersection_property(intersection_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    Ok(XmlNode::new(CityGmlElement::Railway.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlRailway {
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

impl From<&Railway> for GmlRailway {
    fn from(item: &Railway) -> Self {
        Self {
            class: item.class().map(RailwayClassValue::code).map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(RailwayFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(RailwayUsageValue::code)
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
    fn test_deserialize_basic_railway() {
        let xml_document = b"
    <tran:Railway gml:id=\"UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f\">
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
    </tran:Railway>";

        let railway = deserialize_railway(xml_document).expect("should work");

        assert_eq!(
            railway.feature_id(),
            &Id::try_from("UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f").expect("should work")
        );

        assert!(railway.lod2_multi_surface().is_none());
        assert_eq!(railway.generic_attributes().len(), 1);
        assert!(railway.intersections().is_empty());
        assert_eq!(railway.sections().len(), 1);
        let traffic_space = railway.sections().first().unwrap().object().unwrap();
        assert_eq!(
            traffic_space.feature_id(),
            &Id::try_from("UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29").expect("should work")
        );
    }
}
