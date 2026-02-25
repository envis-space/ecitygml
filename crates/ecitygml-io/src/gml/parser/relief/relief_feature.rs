use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_space_boundary;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use ecitygml_core::model::relief::{ReliefComponentKind, ReliefFeature};
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub fn parse_relief_feature(xml_document: &[u8]) -> Result<ReliefFeature, Error> {
    let abstract_space_boundary = parse_abstract_space_boundary(xml_document)?;
    let gml_relief_feature: GmlReliefFeature = de::from_reader(xml_document)?;
    let mut relief_feature =
        ReliefFeature::new(abstract_space_boundary, gml_relief_feature.lod.try_into()?);

    let parsed_city_objects =
        read_city_objects(xml_document, HashSet::from([CityObjectClass::TinRelief]))?;
    for x in parsed_city_objects {
        match x {
            CityObjectKind::TinRelief(x) => {
                relief_feature
                    .relief_component_mut()
                    .push(ReliefComponentKind::TinRelief(x));
            }
            _ => {
                dbg!("unsupported city object: {:?}", x);
            }
        }
    }

    Ok(relief_feature)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlReliefFeature {
    pub lod: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::common::LevelOfDetail;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;

    #[test]
    fn test_parse_tin_relief() {
        let xml_document = "
            <dem:ReliefFeature gml:id=\"ID_7a8b707e-f87c-35f3-8e3c-254427e59493\">
              <dem:lod>2</dem:lod>
              <dem:reliefComponent>
                <dem:TINRelief gml:id=\"abc\">
                  <dem:lod>3</dem:lod>
                    <dem:tin>
                    <gml:TriangulatedSurface gml:id=\"ground\">
                      <gml:patches>
                        <gml:Triangle>
                          <gml:exterior>
                            <gml:LinearRing gml:id=\"Geo28109128\">
                              <gml:posList>513037.352708 5403284.890495 247.27 513034.424098 5403297.424959 247.19 513034.423981 5403287.429362 247.3 513037.352708 5403284.890495 247.27</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Triangle>
                      </gml:patches>
                    </gml:TriangulatedSurface>
                    </dem:tin>
                </dem:TINRelief>
              </dem:reliefComponent>
            </dem:ReliefFeature>";

        let relief_feature = parse_relief_feature(xml_document.as_bytes()).expect("should work");

        assert_eq!(
            relief_feature.id(),
            &Id::try_from("ID_7a8b707e-f87c-35f3-8e3c-254427e59493").unwrap()
        );
        assert_eq!(relief_feature.lod(), LevelOfDetail::Two);
        assert_eq!(relief_feature.num_relief_components(), 1);
    }
}
