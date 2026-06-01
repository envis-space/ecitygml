use crate::Error;
use crate::gml::codec::core::deserialize_abstract_thematic_surface;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::transportation::TrafficArea;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_traffic_area(xml_document: &[u8]) -> Result<TrafficArea, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_thematic_surface_result, parsed_result) = rayon::join(
        || deserialize_abstract_thematic_surface(xml_document, &spans),
        || de::from_reader::<_, GmlTrafficArea>(xml_document).map_err(Error::from),
    );
    let abstract_thematic_surface = abstract_thematic_surface_result?;
    let parsed = parsed_result?;
    let mut traffic_area = TrafficArea::new(abstract_thematic_surface);

    traffic_area.set_class(parsed.class.map(|x| x.into()));
    traffic_area.set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    traffic_area.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());
    traffic_area.set_surface_material(parsed.surface_material.map(|x| x.into()));

    Ok(traffic_area)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficArea {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "tran:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "tran:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,

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
    fn test_deserialize_basic_traffic_area() {
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

        let traffic_area = deserialize_traffic_area(xml_document).expect("should work");

        assert_eq!(
            traffic_area.id(),
            &Id::try_from("UUID_482ae9d8-0d5f-3a97-9d9e-e5362caa2a57").expect("should work")
        );
        assert!(traffic_area.lod2_multi_surface().is_none());
        assert_eq!(traffic_area.generic_attributes().len(), 1);
        assert_eq!(traffic_area.functions().first().unwrap().value(), "2");
        assert_eq!(traffic_area.usages().first().unwrap().value(), "1");
        assert_eq!(traffic_area.surface_material().unwrap().value(), "1");
    }
}
