use crate::Error;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::transportation::AuxiliaryTrafficArea;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_auxiliary_traffic_area(xml_document: &[u8]) -> Result<AuxiliaryTrafficArea, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let mut auxiliary_traffic_area = AuxiliaryTrafficArea::new(abstract_thematic_surface);
    let parsed_result: GmlAuxiliaryTrafficArea = de::from_reader(xml_document)?;

    auxiliary_traffic_area.set_function(
        parsed_result
            .function
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    auxiliary_traffic_area.set_usage(parsed_result.usage.into_iter().map(|x| x.into()).collect());
    auxiliary_traffic_area.set_surface_material(parsed_result.surface_material.map(|x| x.into()));

    Ok(auxiliary_traffic_area)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAuxiliaryTrafficArea {
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
            b"<tran:AuxiliaryTrafficArea gml:id=\"UUID_312e01da-5c89-3f89-b8c0-3e0a8bafecbb\">
                  <genericAttribute>
                    <gen:StringAttribute>
                      <gen:name>opendrive_lane_type</gen:name>
                      <gen:value>BORDER</gen:value>
                    </gen:StringAttribute>
                  </genericAttribute>
                </tran:AuxiliaryTrafficArea>";

        let auxiliary_traffic_area =
            parse_auxiliary_traffic_area(xml_document).expect("should work");

        assert_eq!(
            auxiliary_traffic_area.id(),
            &Id::try_from("UUID_312e01da-5c89-3f89-b8c0-3e0a8bafecbb").expect("should work")
        );
        assert!(auxiliary_traffic_area.lod2_multi_surface().is_none());
        assert_eq!(auxiliary_traffic_area.generic_attributes().len(), 1);
    }
}
