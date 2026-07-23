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
    TrackClassValue, TrackFunctionValue, TrackUsageValue,
};
use ecitygml_core::model::transportation::{AsAbstractTransportationSpace, Track};
use egml::io::codec::basic::GmlCode;
use egml::io::util::collect_children;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

pub fn deserialize_track(xml_document: &[u8]) -> Result<Track, Error> {
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
            parsed_result =
                Some(quick_xml::de::from_reader::<_, GmlTrack>(xml_document).map_err(Error::from));
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

    let mut track = Track::from_abstract_transportation_space(abstract_transportation_space);
    track.set_class_opt(parsed.class.map(Code::from).map(TrackClassValue::from));
    track.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(TrackFunctionValue::from)
            .collect(),
    );
    track.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(TrackUsageValue::from)
            .collect(),
    );
    track.set_sections(sections);
    track.set_intersections(intersections);

    Ok(track)
}

pub fn serialize_track(track: &Track, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_transportation_space(track.abstract_transportation_space(), formatting)?;

    if let Some(raw) = serialize_inner(GmlTrack::from(track), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for section_property in track.sections() {
        let node = serialize_section_property(section_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    for intersection_property in track.intersections() {
        let node = serialize_intersection_property(intersection_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    Ok(XmlNode::new(CityGmlElement::Track.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrack {
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

impl From<&Track> for GmlTrack {
    fn from(item: &Track) -> Self {
        Self {
            class: item.class().map(TrackClassValue::code).map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(TrackFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(TrackUsageValue::code)
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
    fn test_deserialize_basic_track() {
        let xml_document = b"
    <tran:Track gml:id=\"UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f\">
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
    </tran:Track>";

        let track = deserialize_track(xml_document).expect("should work");

        assert_eq!(
            track.feature_id(),
            &Id::try_from("UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f").expect("should work")
        );

        assert!(track.lod2_multi_surface().is_none());
        assert_eq!(track.generic_attributes().len(), 1);
        assert!(track.intersections().is_empty());
        assert_eq!(track.sections().len(), 1);
        let traffic_space = track.sections().first().unwrap().object().unwrap();
        assert_eq!(
            traffic_space.feature_id(),
            &Id::try_from("UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29").expect("should work")
        );
    }
}
