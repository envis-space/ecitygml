use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_space;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use ecitygml_core::model::transportation::Intersection;
use std::collections::HashSet;

pub fn parse_intersection(xml_document: &[u8]) -> Result<Intersection, Error> {
    let space = parse_abstract_space(xml_document)?;
    let mut intersection = Intersection::new(space);

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
                intersection.auxiliary_traffic_space.push(x);
            }
            CityObjectKind::TrafficSpace(x) => {
                intersection.traffic_space.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(intersection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace};
    use ecitygml_core::model::transportation::GranularityValue;
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_intersection() {
        let xml_document = b"
        <tran:Intersection gml:id=\"UUID_9a9fc5a0-b252-3d63-ac79-b3141175f152\">
          <genericAttribute>
            <gen:StringAttribute>
              <gen:name>identifier_new_section</gen:name>
              <gen:value>abc</gen:value>
            </gen:StringAttribute>
          </genericAttribute>
          <tran:trafficSpace>
            <tran:TrafficSpace gml:id=\"UUID_ff91145b-98e8-388b-b4d1-b94624f806db\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>identifier_roadId</gen:name>
                  <gen:value>4516050</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <tran:function>2</tran:function>
              <tran:usage>1</tran:usage>
              <tran:granularity>lane</tran:granularity>
              <tran:trafficDirection>forwards</tran:trafficDirection>
              <tran:predecessor xlink:href=\"#UUID_35e15191-e911-320b-bbe7-004b237a024a\"/>
              <tran:successor xlink:href=\"#UUID_efb6f11e-0d92-3d1f-8208-7ddbea02e829\"/>
            </tran:TrafficSpace>
          </tran:trafficSpace>
        </tran:Intersection>";

        let intersection = parse_intersection(xml_document).expect("should work");

        assert_eq!(
            intersection.id(),
            &Id::try_from("UUID_9a9fc5a0-b252-3d63-ac79-b3141175f152").expect("should work")
        );

        assert!(intersection.lod2_multi_surface().is_none());
        assert_eq!(intersection.generic_attributes().len(), 1);
        assert!(intersection.auxiliary_traffic_space.is_empty());
        assert_eq!(intersection.traffic_space.len(), 1);
        let traffic_space = intersection.traffic_space.first().unwrap();

        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_ff91145b-98e8-388b-b4d1-b94624f806db").expect("should work")
        );
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);
        assert!(traffic_space.traffic_area.is_empty());
    }
}
