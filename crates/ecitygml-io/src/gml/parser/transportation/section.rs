use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_space;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use ecitygml_core::model::transportation::Section;
use std::collections::HashSet;

pub fn parse_section(xml_document: &[u8]) -> Result<Section, Error> {
    let abstract_space = parse_abstract_space(xml_document)?;
    let mut section = Section::new(abstract_space);

    let parsed_city_objects = read_city_objects(
        xml_document,
        HashSet::from([
            CityObjectClass::AuxiliaryTrafficSpace,
            CityObjectClass::TrafficSpace,
        ]),
    )?;
    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::AuxiliaryTrafficSpace(x) => {
                section.auxiliary_traffic_space.push(x);
            }
            CityObjectKind::TrafficSpace(x) => {
                section.traffic_space.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(section)
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

        let section = parse_section(xml_document).expect("should work");

        assert_eq!(
            section.id(),
            &Id::try_from("UUID_ef20f165-2564-373a-a5f8-98fd15f1ae69").expect("should work")
        );

        assert!(section.lod2_multi_surface().is_none());
        assert_eq!(section.generic_attributes().len(), 1);
        assert!(section.auxiliary_traffic_space.is_empty());
        assert_eq!(section.traffic_space.len(), 1);
        let traffic_space = section.traffic_space.first().unwrap();

        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_9582b02f-5cc7-38f8-b79c-a3a18b31856c").expect("should work")
        );
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);
        assert!(traffic_space.traffic_area.is_empty());
    }
}
