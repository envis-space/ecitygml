use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_thematic_surface;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::construction::WallSurface;
use ecitygml_core::model::core::CityObjectKind;
use std::collections::HashSet;

pub fn parse_wall_surface(xml_document: &[u8]) -> Result<WallSurface, Error> {
    let abstract_thematic_surface = parse_abstract_thematic_surface(xml_document)?;
    let mut wall_surface = WallSurface::new(abstract_thematic_surface);

    let parsed_city_objects = read_city_objects(
        xml_document,
        HashSet::from([CityObjectClass::DoorSurface, CityObjectClass::WindowSurface]),
    )?;

    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::DoorSurface(x) => {
                wall_surface.door_surface.push(x);
            }
            CityObjectKind::WindowSurface(x) => {
                wall_surface.window_surface.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(wall_surface)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::parser::core::parse_abstract_thematic_surface;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_parse_basic_abstract_thematic_surface() {
        let xml_document = b"
<con:WallSurface gml:id=\"GML_d38cf762-c29d-4491-88c9-bdc89e141978\">
          <gml:name>Outer Wall 2 (South)</gml:name>
          <lod3MultiSurface>
            <gml:MultiSurface>
              <gml:surfaceMember>
                <gml:CompositeSurface gml:id=\"GML_4726d5c0-dfa2-4777-b1da-24798d72c27a\">
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58807_717_125437_84247\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58807_717_125437_84247_0\">
                          <gml:pos>457849.005 5439083.0 114.375</gml:pos>
                          <gml:pos>457849.005 5439083.2 114.375</gml:pos>
                          <gml:pos>457849.005 5439083.2 112.0</gml:pos>
                          <gml:pos>457849.005 5439083.0 112.0</gml:pos>
                          <gml:pos>457849.005 5439083.0 114.375</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58808_349_692294_125678\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58808_349_692294_125678_0\">
                          <gml:pos>457852.21 5439083.0 114.0</gml:pos>
                          <gml:pos>457852.21 5439083.12 114.0</gml:pos>
                          <gml:pos>457852.21 5439083.12 112.8</gml:pos>
                          <gml:pos>457852.21 5439083.0 112.8</gml:pos>
                          <gml:pos>457852.21 5439083.0 114.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58809_472_527501_416856\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58809_472_527501_416856_0\">
                          <gml:pos>457845.79 5439083.0 114.0</gml:pos>
                          <gml:pos>457845.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457845.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.0 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.0 114.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58810_1807_553097_148846\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58810_1807_553097_148846_0\">
                          <gml:pos>457845.79 5439083.0 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457843.79 5439083.0 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.0 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58811_1622_73903_56220\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58811_1622_73903_56220_0\">
                          <gml:pos>457846.995 5439083.0 114.375</gml:pos>
                          <gml:pos>457846.995 5439083.2 114.375</gml:pos>
                          <gml:pos>457849.005 5439083.2 114.375</gml:pos>
                          <gml:pos>457849.005 5439083.0 114.375</gml:pos>
                          <gml:pos>457846.995 5439083.0 114.375</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58812_795_350114_216214\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58812_795_350114_216214_0\">
                          <gml:pos>457843.79 5439083.0 112.8</gml:pos>
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457843.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.0 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.0 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58813_1099_461650_222485\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58813_1099_461650_222485_0\">
                          <gml:pos>457846.995 5439083.0 112.0</gml:pos>
                          <gml:pos>457846.995 5439083.2 112.0</gml:pos>
                          <gml:pos>457846.995 5439083.2 114.375</gml:pos>
                          <gml:pos>457846.995 5439083.0 114.375</gml:pos>
                          <gml:pos>457846.995 5439083.0 112.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58814_1459_649731_52436\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58814_1459_649731_52436_0\">
                          <gml:pos>457850.21 5439083.0 114.0</gml:pos>
                          <gml:pos>457850.21 5439083.12 114.0</gml:pos>
                          <gml:pos>457852.21 5439083.12 114.0</gml:pos>
                          <gml:pos>457852.21 5439083.0 114.0</gml:pos>
                          <gml:pos>457850.21 5439083.0 114.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58815_691_101880_418020\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58815_691_101880_418020_0\">
                          <gml:pos>457850.21 5439083.0 112.8</gml:pos>
                          <gml:pos>457850.21 5439083.12 112.8</gml:pos>
                          <gml:pos>457850.21 5439083.12 114.0</gml:pos>
                          <gml:pos>457850.21 5439083.0 114.0</gml:pos>
                          <gml:pos>457850.21 5439083.0 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58816_858_312337_86583\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58816_858_312337_86583_0\">
                          <gml:pos>457846.995 5439083.0 112.0</gml:pos>
                          <gml:pos>457849.005 5439083.0 112.0</gml:pos>
                          <gml:pos>457849.005 5439083.2 112.0</gml:pos>
                          <gml:pos>457846.995 5439083.2 112.0</gml:pos>
                          <gml:pos>457846.995 5439083.0 112.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58817_701_101369_361161\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58817_701_101369_361161_0\">
                          <gml:pos>457852.21 5439083.0 112.8</gml:pos>
                          <gml:pos>457852.21 5439083.12 112.8</gml:pos>
                          <gml:pos>457850.21 5439083.12 112.8</gml:pos>
                          <gml:pos>457850.21 5439083.0 112.8</gml:pos>
                          <gml:pos>457852.21 5439083.0 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58818_1640_464682_59215\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58818_1640_464682_59215_0\">
                          <gml:pos>457843.79 5439083.0 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457845.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457845.79 5439083.0 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.0 114.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58819_65_364244_211813\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58819_65_364244_211813_0\">
                          <gml:pos>457854.0 5439083.0 115.430940107676</gml:pos>
                          <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                          <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                          <gml:pos>457854.0 5439083.0 111.8</gml:pos>
                          <gml:pos>457854.0 5439083.0 115.430940107676</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                      <gml:interior>
                        <gml:LinearRing gml:id=\"PolyID58819_65_364244_211813_1\">
                          <gml:pos>457849.005 5439083.0 114.375</gml:pos>
                          <gml:pos>457849.005 5439083.0 112.0</gml:pos>
                          <gml:pos>457846.995 5439083.0 112.0</gml:pos>
                          <gml:pos>457846.995 5439083.0 114.375</gml:pos>
                          <gml:pos>457849.005 5439083.0 114.375</gml:pos>
                        </gml:LinearRing>
                      </gml:interior>
                      <gml:interior>
                        <gml:LinearRing gml:id=\"PolyID58819_65_364244_211813_2\">
                          <gml:pos>457850.21 5439083.0 114.0</gml:pos>
                          <gml:pos>457852.21 5439083.0 114.0</gml:pos>
                          <gml:pos>457852.21 5439083.0 112.8</gml:pos>
                          <gml:pos>457850.21 5439083.0 112.8</gml:pos>
                          <gml:pos>457850.21 5439083.0 114.0</gml:pos>
                        </gml:LinearRing>
                      </gml:interior>
                      <gml:interior>
                        <gml:LinearRing gml:id=\"PolyID58819_65_364244_211813_3\">
                          <gml:pos>457845.79 5439083.0 112.8</gml:pos>
                          <gml:pos>457843.79 5439083.0 112.8</gml:pos>
                          <gml:pos>457843.79 5439083.0 114.0</gml:pos>
                          <gml:pos>457845.79 5439083.0 114.0</gml:pos>
                          <gml:pos>457845.79 5439083.0 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:interior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:CompositeSurface>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod3MultiSurface>
          <con:fillingSurface>
            <con:WindowSurface gml:id=\"GML_98d9c4f5-9e47-4f0b-95f3-cf31e7520142\">
              <gml:name>Window East</gml:name>
              <lod3MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58820_1568_227087_210505\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58820_1568_227087_210505_0\">
                          <gml:pos>457852.21 5439083.12 112.8</gml:pos>
                          <gml:pos>457852.21 5439083.12 114.0</gml:pos>
                          <gml:pos>457850.21 5439083.12 114.0</gml:pos>
                          <gml:pos>457850.21 5439083.12 112.8</gml:pos>
                          <gml:pos>457852.21 5439083.12 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod3MultiSurface>
            </con:WindowSurface>
          </con:fillingSurface>
          <con:fillingSurface>
            <con:WindowSurface gml:id=\"GML_d0f329f3-5b05-428d-87c3-945b3868337f\">
              <gml:name>Window West</gml:name>
              <lod3MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58821_1939_612838_272028\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58821_1939_612838_272028_0\">
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod3MultiSurface>
            </con:WindowSurface>
          </con:fillingSurface>
          <con:fillingSurface>
            <con:DoorSurface gml:id=\"GML_2d6ddf04-ee56-42a1-a9b1-b47e4181a629\">
              <gml:name>Door South</gml:name>
              <lod3MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58822_551_84845_215911\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58822_551_84845_215911_0\">
                          <gml:pos>457849.005 5439083.2 112.0</gml:pos>
                          <gml:pos>457849.005 5439083.2 114.375</gml:pos>
                          <gml:pos>457846.995 5439083.2 114.375</gml:pos>
                          <gml:pos>457846.995 5439083.2 112.0</gml:pos>
                          <gml:pos>457849.005 5439083.2 112.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod3MultiSurface>
            </con:DoorSurface>
          </con:fillingSurface>
        </con:WallSurface>";

        let wall_surface = parse_wall_surface(xml_document).expect("should work");

        assert_eq!(
            wall_surface.id(),
            &Id::try_from("GML_d38cf762-c29d-4491-88c9-bdc89e141978").expect("should work")
        );
        assert!(wall_surface.lod3_multi_surface().is_some());
        assert_eq!(wall_surface.generic_attributes().len(), 0);
        assert_eq!(wall_surface.window_surface.len(), 2);
        assert_eq!(wall_surface.door_surface.len(), 1);
    }
}
