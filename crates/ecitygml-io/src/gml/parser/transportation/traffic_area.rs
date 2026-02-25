use crate::Error;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::transportation::TrafficArea;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_traffic_area(xml_document: &[u8]) -> Result<TrafficArea, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let mut traffic_area = TrafficArea::new(abstract_thematic_surface);
    let parsed_result: GmlTrafficArea = de::from_reader(xml_document)?;

    traffic_area.set_function(
        parsed_result
            .function
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    traffic_area.set_usage(parsed_result.usage.into_iter().map(|x| x.into()).collect());
    traffic_area.set_surface_material(parsed_result.surface_material.map(|x| x.into()));

    Ok(traffic_area)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficArea {
    #[serde(rename = "function", default)]
    pub function: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usage: Vec<GmlCode>,

    #[serde(rename = "surfaceMaterial")]
    pub surface_material: Option<GmlCode>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_traffic_area() {
        let xml_document =
            b"<tran:TrafficArea gml:id=\"UUID_482ae9d8-0d5f-3a97-9d9e-e5362caa2a57\">
                  <gml:name>Lane</gml:name>
                  <genericAttribute>
                    <gen:StringAttribute>
                      <gen:name>opendrive_lane_type</gen:name>
                      <gen:value>SIDEWALK</gen:value>
                    </gen:StringAttribute>
                  </genericAttribute>
                  <tran:function>2</tran:function>
                  <tran:usage>1</tran:usage>
                  <tran:surfaceMaterial>1</tran:surfaceMaterial>
                </tran:TrafficArea>";

        let traffic_area = parse_traffic_area(xml_document).expect("should work");

        assert_eq!(
            traffic_area.id(),
            &Id::try_from("UUID_482ae9d8-0d5f-3a97-9d9e-e5362caa2a57").expect("should work")
        );
        assert!(traffic_area.lod2_multi_surface().is_none());
        assert_eq!(traffic_area.generic_attributes().len(), 1);
        assert_eq!(traffic_area.function().first().unwrap().value, "2");
        assert_eq!(traffic_area.usage().first().unwrap().value, "1");
        assert_eq!(traffic_area.surface_material().unwrap().value, "1");
    }
}
