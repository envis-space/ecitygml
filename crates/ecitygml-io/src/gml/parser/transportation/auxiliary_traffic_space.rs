use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_space;
use crate::gml::parser::transportation::granularity_value::GmlGranularityValue;
use crate::gml::parser::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use ecitygml_core::model::transportation::AuxiliaryTrafficSpace;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub fn parse_auxiliary_traffic_space(xml_document: &[u8]) -> Result<AuxiliaryTrafficSpace, Error> {
    let abstract_space = parse_abstract_space(xml_document)?;
    let parsed_result: GmlAuxiliaryTrafficSpace = de::from_reader(xml_document)?;

    let mut traffic_space =
        AuxiliaryTrafficSpace::new(abstract_space, parsed_result.granularity.into());

    traffic_space.set_function(
        parsed_result
            .function
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    traffic_space.set_usage(parsed_result.usage.into_iter().map(|x| x.into()).collect());

    let parsed_city_objects = read_city_objects(
        xml_document,
        HashSet::from([CityObjectClass::AuxiliaryTrafficArea]),
    )?;
    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::AuxiliaryTrafficArea(x) => {
                traffic_space.auxiliary_traffic_area.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(traffic_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAuxiliaryTrafficSpace {
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
    use ecitygml_core::model::core::{AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace};
    use ecitygml_core::model::transportation::{GranularityValue, TrafficDirectionValue};
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_traffic_area() {
        let xml_document =
            b"<tran:AuxiliaryTrafficSpace gml:id=\"UUID_24f88e8d-34c8-3a4f-8889-dff3a82f5121\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>opendrive_lane_type</gen:name>
                  <gen:value>BORDER</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <boundary>
                <tran:AuxiliaryTrafficArea gml:id=\"UUID_caa7268d-8c82-3595-837f-0584a6c261ec\">
                  <gml:name>Lane</gml:name>
                  <genericAttribute>
                    <gen:IntAttribute>
                      <gen:name>identifier_laneId</gen:name>
                      <gen:value>2</gen:value>
                    </gen:IntAttribute>
                  </genericAttribute>
                  <genericAttribute>
                    <gen:StringAttribute>
                      <gen:name>identifier_roadId</gen:name>
                      <gen:value>1970000</gen:value>
                    </gen:StringAttribute>
                  </genericAttribute>
                  <tran:surfaceMaterial>10</tran:surfaceMaterial>
                </tran:AuxiliaryTrafficArea>
              </boundary>
              <lod2MultiCurve>
                <gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">678289.3084552324 5403784.921937843 366.9414 678288.9147169285 5403785.472392212 366.94323252854576 678288.5189885574 5403786.020235433 366.94503322674285 678288.121192659 5403786.565393211 366.9467997078538 678287.7212541931 5403787.107788266 366.9485295851412 678287.3191884857 5403787.64721126 366.95022047186745 678286.9149447819 5403788.183504009 366.9518699812952 678286.5084192362 5403788.716575279 366.9534757266869 678286.0995073881 5403789.246335832 366.9550353213051 678285.6881040192 5403789.772698513 366.95654637841227 678285.2741030005 5403790.295578343 366.95800651127104 678284.8574651208 5403790.814803777 366.9594133331438 678284.4381611495 5403791.330162254 366.9607644572932 678284.016099949 5403791.841516722 366.9620574969817 678283.591337201 5403792.34886217 366.96329006547177 678283.1641803615 5403792.85243178 366.96445977602605 678282.7349477345 5403793.352500112 366.965564241907 678282.3039399333 5403793.849361616 366.9666010763771 678281.8714379214 5403794.343331684 366.967567892699 678281.4376095616 5403794.834659517 366.9684623041351 678281.0011907143 5403795.323983301 366.96928192394796 678280.5562650429 5403795.817246481 366.97002436540015 678280.1023219397 5403796.315918498 366.9706872417541 678279.6400387053 5403796.8203841355 366.9712681662724 678279.1696820334 5403797.330557862 366.9717647522176 678278.6907719106 5403797.8455672255 366.9721746128522 678278.2027770189 5403798.364524244 366.9724953614387 678277.7051508029 5403798.886640374 366.9727246112396 678277.1980010003 5403799.4107006015 366.9728599755175 678276.688090822 5403799.93037767 366.97289906753485 678276.177511188 5403800.444484803 366.9728395005542 678275.6666084244 5403800.953525639 366.9726788878381 678275.1557869348 5403801.457970989 366.972414842649 678274.6455299138 5403801.958291686 366.9720449782495 678274.1364455012 5403802.455014871 366.9715669079022 678273.6292184186 5403802.948691789 366.9709782448694 678273.1244556711 5403803.439973384 366.9702766024138 678272.6211834472 5403803.931515212 366.9694595937978 678272.1198059253 5403804.425038017 366.96852483228406 678271.6212901035 5403804.921444074 366.967469931135 678271.1263741058 5403805.421417236 366.9662925036132 678270.6357456157 5403805.925589678 366.9649901629811 678270.1499070801 5403806.434416919 366.9635605225014 678269.669291459 5403806.948285976 366.9620011954364 678269.1941272602 5403807.467825568 366.9603097950488 678269.0961093021 5403807.576547665 366.95994166800944</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>
              </lod2MultiCurve>
              <tran:granularity>lane</tran:granularity>
            </tran:AuxiliaryTrafficSpace>";

        let auxiliary_traffic_space =
            parse_auxiliary_traffic_space(xml_document).expect("should work");

        assert_eq!(
            auxiliary_traffic_space.id(),
            &Id::try_from("UUID_24f88e8d-34c8-3a4f-8889-dff3a82f5121").expect("should work")
        );
        assert!(auxiliary_traffic_space.lod2_multi_surface().is_none());
        assert_eq!(auxiliary_traffic_space.generic_attributes().len(), 1);
        assert_eq!(
            auxiliary_traffic_space.granularity(),
            &GranularityValue::Lane
        );
        assert_eq!(auxiliary_traffic_space.auxiliary_traffic_area.len(), 1);

        let traffic_area = auxiliary_traffic_space
            .auxiliary_traffic_area
            .first()
            .unwrap();
        assert_eq!(
            traffic_area.id(),
            &Id::try_from("UUID_caa7268d-8c82-3595-837f-0584a6c261ec").expect("should work")
        );
        assert_eq!(traffic_area.generic_attributes().len(), 2);
    }
}
