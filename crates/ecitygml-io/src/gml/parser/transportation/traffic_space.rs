use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_space;
use crate::gml::parser::transportation::granularity_value::GmlGranularityValue;
use crate::gml::parser::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use ecitygml_core::model::transportation::TrafficSpace;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub fn parse_traffic_space(xml_document: &[u8]) -> Result<TrafficSpace, Error> {
    let abstract_space = parse_abstract_space(xml_document)?;
    let parsed_result: GmlTrafficSpace = de::from_reader(xml_document)?;

    let mut traffic_space = TrafficSpace::new(abstract_space, parsed_result.granularity.into());

    traffic_space.set_function(
        parsed_result
            .function
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    traffic_space.set_usage(parsed_result.usage.into_iter().map(|x| x.into()).collect());
    traffic_space.set_traffic_direction(parsed_result.traffic_direction.map(|x| x.into()));

    let parsed_city_objects =
        read_city_objects(xml_document, HashSet::from([CityObjectClass::TrafficArea]))?;
    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::TrafficArea(x) => {
                traffic_space.traffic_area.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(traffic_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficSpace {
    #[serde(rename = "function", default)]
    pub function: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usage: Vec<GmlCode>,

    #[serde(rename = "granularity", default)]
    pub granularity: GmlGranularityValue,

    #[serde(rename = "trafficDirection", default)]
    pub traffic_direction: Option<GmlTrafficDirectionValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, SpaceType,
    };
    use ecitygml_core::model::transportation::{GranularityValue, TrafficDirectionValue};
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_section() {
        let xml_document =
            b"<tran:TrafficSpace gml:id=\"UUID_6e4de408-1e54-3869-b7ce-1be3f2261421\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>opendrive_lane_type</gen:name>
                  <gen:value>DRIVING</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <spaceType>open</spaceType>
              <boundary>
                <tran:TrafficArea gml:id=\"UUID_dc110e80-dadc-3c87-b864-2854cc0cb39a\">
                  <gml:name>Lane</gml:name>
                  <genericAttribute>
                    <gen:IntAttribute>
                      <gen:name>identifier_laneId</gen:name>
                      <gen:value>-1</gen:value>
                    </gen:IntAttribute>
                  </genericAttribute>
                  <tran:function>1</tran:function>
                  <tran:usage>2</tran:usage>
                </tran:TrafficArea>
              </boundary>
              <lod2MultiCurve>
                <gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">-52.6446227593087 1.5877425640718097 0.0 -47.64466181585868 1.6075052555478968 0.0 -42.64470087240866 1.627267947023984 0.0 -37.64473992895864 1.6470306385000706 0.0 -32.64477898550862 1.6667933299761577 0.0 -27.6448180420586 1.6865560214522448 0.0 -22.64485709860858 1.706318712928332 0.0 -17.644896155158555 1.7260814044044186 0.0 -15.898072287268029 1.7329858465699939 0.0</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>
              </lod2MultiCurve>
              <tran:function>1</tran:function>
              <tran:usage>2</tran:usage>
              <tran:granularity>lane</tran:granularity>
              <tran:trafficDirection>forwards</tran:trafficDirection>
              <tran:successor xlink:href=\"#UUID_144a6807-5844-32b2-bb34-8b2671b1afaa\"/>
            </tran:TrafficSpace>";

        let traffic_space = parse_traffic_space(xml_document).expect("should work");

        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_6e4de408-1e54-3869-b7ce-1be3f2261421").expect("should work")
        );
        assert!(traffic_space.lod2_multi_surface().is_none());
        assert_eq!(traffic_space.generic_attributes().len(), 1);
        assert_eq!(traffic_space.space_type(), Some(&SpaceType::Open));
        assert_eq!(traffic_space.function().first().unwrap().value, "1");
        assert_eq!(traffic_space.usage().first().unwrap().value, "2");
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);
        assert_eq!(
            traffic_space.traffic_direction().unwrap(),
            TrafficDirectionValue::Forwards
        );
        assert_eq!(traffic_space.traffic_area.len(), 1);

        let traffic_area = traffic_space.traffic_area.first().unwrap();
        assert_eq!(
            traffic_area.id(),
            &Id::try_from("UUID_dc110e80-dadc-3c87-b864-2854cc0cb39a").expect("should work")
        );
        assert_eq!(traffic_area.generic_attributes().len(), 1);
        assert_eq!(traffic_area.function().first().unwrap().value, "1");
        assert_eq!(traffic_area.usage().first().unwrap().value, "2");
    }
}
