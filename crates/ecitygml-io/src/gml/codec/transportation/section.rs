use crate::Error;
use crate::gml::codec::transportation::GmlIntersection;
use crate::gml::codec::transportation::abstract_transportation_space::{
    deserialize_abstract_transportation_space, serialize_abstract_transportation_space,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::transportation::values::SectionClassValue;
use ecitygml_core::model::transportation::{AsAbstractTransportationSpace, Section};
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_section(xml_document: &[u8]) -> Result<Section, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_transportation_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_transportation_space(xml_document, &spans),
        || de::from_reader::<_, GmlIntersection>(xml_document).map_err(Error::from),
    );
    let abstract_transportation_space = abstract_transportation_space_result?;
    let parsed = parsed_result?;

    let mut section = Section::from_abstract_transportation_space(abstract_transportation_space);
    section.set_class_opt(parsed.class.map(Code::from).map(SectionClassValue::from));

    Ok(section)
}

pub fn serialize_section(section: &Section, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_transportation_space(
        section.abstract_transportation_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(GmlSection::from(section), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(CityGmlElement::Section.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSection {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,
}

impl From<&Section> for GmlSection {
    fn from(item: &Section) -> Self {
        Self {
            class: item.class().map(SectionClassValue::code).map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AbstractSpaceBoundaryKind, AbstractThematicSurfaceKind, AsAbstractCityObject,
        AsAbstractFeature, AsAbstractSpace,
    };
    use ecitygml_core::model::transportation::{
        AsAbstractTransportationSpace, GranularityValue, TrafficArea, TrafficDirectionValue,
    };
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_traffic_area() {
        let xml_document =
            b"<tran:Section gml:id=\"UUID_ef20f165-2564-373a-a5f8-98fd15f1ae69\">
          <genericAttribute>
            <gen:StringAttribute>
              <gen:name>identifier_new_section</gen:name>
              <gen:value>abc</gen:value>
            </gen:StringAttribute>
          </genericAttribute>
          <tran:trafficSpace>
            <tran:TrafficSpace gml:id=\"UUID_9582b02f-5cc7-38f8-b79c-a3a18b31856c\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>identifier_roadId</gen:name>
                  <gen:value>2</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <spaceType>open</spaceType>
              <lod2MultiCurve>
                <gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">14.101693373432088 1.8515619954265157 0.0 19.101654316882108 1.8713246869026023 0.0 24.10161526033213 1.8910873783786895 0.0 29.10157620378215 1.9108500698547761 0.0 34.10153714723217 1.9306127613308632 0.0</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>
              </lod2MultiCurve>
              <tran:function>1</tran:function>
              <tran:usage>2</tran:usage>
              <tran:granularity>lane</tran:granularity>
              <tran:trafficDirection>forwards</tran:trafficDirection>
              <tran:predecessor xlink:href=\"#UUID_656f23c9-92b6-3c98-a1bf-40b5f2625b13\"/>
              <tran:predecessor xlink:href=\"#UUID_86f8df09-c972-332b-a279-636373ce4e0c\"/>
            </tran:TrafficSpace>
          </tran:trafficSpace>
        </tran:Section>";

        let section = deserialize_section(xml_document).expect("should work");

        assert_eq!(
            section.feature_id(),
            &Id::try_from("UUID_ef20f165-2564-373a-a5f8-98fd15f1ae69").expect("should work")
        );

        assert!(section.lod2_multi_surface().is_none());
        assert_eq!(section.generic_attributes().len(), 1);
        assert!(section.auxiliary_traffic_spaces().is_empty());
        assert_eq!(section.traffic_spaces().len(), 1);
        let traffic_space = section.traffic_spaces().first().unwrap().object().unwrap();

        assert_eq!(
            traffic_space.feature_id(),
            &Id::try_from("UUID_9582b02f-5cc7-38f8-b79c-a3a18b31856c").expect("should work")
        );
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);

        let traffic_areas: Vec<&TrafficArea> = traffic_space
            .boundaries()
            .iter()
            .flat_map(|x| x.object())
            .filter_map(|x| match x {
                AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(
                    AbstractThematicSurfaceKind::TrafficArea(x),
                ) => Some(x),
                _ => None,
            })
            .collect();
        assert!(traffic_areas.is_empty());
    }

    #[test]
    fn test_deserialize_basic_marking() {
        let xml_document = b"<tran:Section gml:id=\"UUID_ef20f165-2564-373a-a5f8-98fd15f1ae69\">
          <tran:marking>
            <tran:Marking gml:id=\"UUID_4828b97d-f847-3ac5-a241-b34920a8e4f0\">
              <gml:name>RoadMarking</gml:name>
              <genericAttribute>
                <gen:IntAttribute>
                  <gen:name>identifier_laneId</gen:name>
                  <gen:value>1</gen:value>
                </gen:IntAttribute>
              </genericAttribute>
              <lod0MultiCurve>
                <gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">678170.5985235515 5403697.532081 367.5595579815167 678171.0511437679 5403697.938116865 367.5664539960987 678171.4276917643 5403698.393826041 367.57329200198734 678171.7194438716 5403698.89136084 367.5800397866049 678171.9267716254 5403699.411845871 367.58666513737376 678172.0514317176 5403699.932105262 367.5931358417163 678172.1062932804 5403700.432366421 367.5994192752446 678172.1012979577 5403700.935231906 367.60542483904044 678172.0293369786 5403701.448128954 367.6109446681859 678171.8816076152 5403701.9590493515 367.6157567490352 678171.656748226 5403702.449175331 367.6196390679427 678171.3686427372 5403702.90921139 367.6223696112627 678171.0523698985 5403703.342731151 367.6237263653494 678170.7387264195 5403703.761125513 367.62348731655726 678170.4384677111 5403704.163595385 367.6214304512404 678170.1523610576 5403704.54694844 367.6173337557533 678169.877680364 5403704.914730365 367.61097521645013 678169.7502549244 5403705.085151579 367.60709999874234</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>
              </lod0MultiCurve>
            </tran:Marking>
          </tran:marking>
        </tran:Section>";

        let section = deserialize_section(xml_document).expect("should work");

        assert_eq!(
            section.feature_id(),
            &Id::try_from("UUID_ef20f165-2564-373a-a5f8-98fd15f1ae69").expect("should work")
        );

        assert!(section.lod2_multi_surface().is_none());
        assert!(section.generic_attributes().is_empty());
        assert_eq!(section.markings().len(), 1);
    }
}
