use crate::Error;
use crate::gml::codec::core::deserialize_abstract_unoccupied_space;
use crate::gml::codec::transportation::granularity_value::GmlGranularityValue;
use crate::gml::codec::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::transportation::TrafficSpace;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_traffic_space(xml_document: &[u8]) -> Result<TrafficSpace, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_unoccupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_unoccupied_space(xml_document, &spans),
        || de::from_reader::<_, GmlTrafficSpace>(xml_document).map_err(Error::from),
    );
    let abstract_unoccupied_space = abstract_unoccupied_space_result?;
    let parsed = parsed_result?;

    let mut traffic_space = TrafficSpace::new(abstract_unoccupied_space, parsed.granularity.into());

    traffic_space.set_class(parsed.class.map(|x| x.into()));
    traffic_space.set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    traffic_space.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());
    traffic_space.set_traffic_direction(parsed.traffic_direction.map(|x| x.into()));

    Ok(traffic_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficSpace {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "tran:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "tran:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,

    #[serde(rename = "granularity", default)]
    pub granularity: GmlGranularityValue,

    #[serde(rename = "trafficDirection", default)]
    pub traffic_direction: Option<GmlTrafficDirectionValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, SpaceBoundaryKind, SpaceType,
        ThematicSurfaceKind,
    };
    use ecitygml_core::model::transportation::{
        GranularityValue, TrafficArea, TrafficDirectionValue,
    };
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_section() {
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

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_6e4de408-1e54-3869-b7ce-1be3f2261421").expect("should work")
        );
        assert!(traffic_space.lod2_multi_surface().is_none());
        assert_eq!(traffic_space.generic_attributes().len(), 1);
        assert_eq!(traffic_space.space_type(), Some(&SpaceType::Open));
        assert_eq!(traffic_space.functions().first().unwrap().value(), "1");
        assert_eq!(traffic_space.usages().first().unwrap().value(), "2");
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);
        assert_eq!(
            traffic_space.traffic_direction().unwrap(),
            TrafficDirectionValue::Forwards
        );
        let traffic_areas: Vec<&TrafficArea> = traffic_space
            .boundaries()
            .iter()
            .flat_map(|x| &x.object)
            .filter_map(|x| match x {
                SpaceBoundaryKind::ThematicSurfaceKind(ThematicSurfaceKind::TrafficArea(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .collect();
        assert_eq!(traffic_areas.len(), 1);

        let traffic_area = traffic_areas.first().unwrap();
        assert_eq!(
            traffic_area.id(),
            &Id::try_from("UUID_dc110e80-dadc-3c87-b864-2854cc0cb39a").expect("should work")
        );
        assert_eq!(traffic_area.generic_attributes().len(), 1);
        assert_eq!(traffic_area.functions().first().unwrap().value(), "1");
        assert_eq!(traffic_area.usages().first().unwrap().value(), "2");
    }
}
