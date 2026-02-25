use crate::error::Error;
use crate::gml::parser::core::parse_abstract_city_object;
use crate::gml::parser::core::space_type::GmlSpaceType;
use ecitygml_core::model::core::{AbstractSpace, AsAbstractFeature, AsAbstractSpaceMut};
use egml::io::aggregates::{GmlMultiCurveProperty, GmlMultiSurfaceProperty};
use egml::io::primitives::GmlSolidProperty;
use egml::model::geometry::aggregates::{MultiCurve, MultiSurface};
use egml::model::geometry::primitives::Solid;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn parse_abstract_space(xml_document: &[u8]) -> Result<AbstractSpace, Error> {
    let abstract_city_object = parse_abstract_city_object(xml_document)?;
    let parsed_result: GmlAbstractSpace = de::from_reader(xml_document)?;

    let mut abstract_space = AbstractSpace::new(abstract_city_object);
    abstract_space.set_space_type(parsed_result.space_type.map(|x| x.into()));

    if let Some(gml_solid_property) = parsed_result.lod1_solid {
        let gml_solid_result: Result<Solid, egml::io::Error> =
            gml_solid_property.content.try_into();

        match gml_solid_result {
            Ok(x) => {
                abstract_space.lod1_solid = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod1Solid of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_solid_property) = parsed_result.lod2_solid {
        let gml_solid_result: Result<Solid, egml::io::Error> =
            gml_solid_property.content.try_into();

        match gml_solid_result {
            Ok(x) => {
                abstract_space.lod2_solid = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod2Solid of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_solid_property) = parsed_result.lod3_solid {
        let gml_solid_result: Result<Solid, egml::io::Error> =
            gml_solid_property.content.try_into();

        match gml_solid_result {
            Ok(x) => {
                abstract_space.lod3_solid = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod3Solid of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_surface_property) = parsed_result.lod0_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_space.lod0_multi_surface = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod0_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_surface_property) = parsed_result.lod2_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_space.lod2_multi_surface = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod2_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_surface_property) = parsed_result.lod3_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_space.lod3_multi_surface = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod3_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_curve_property) = parsed_result.lod0_multi_curve {
        let multi_curve_result: Result<MultiCurve, egml::io::Error> =
            gml_multi_curve_property.content.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_space.lod2_multi_curve = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod0_multi_curve of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_curve_property) = parsed_result.lod2_multi_curve {
        let multi_curve_result: Result<MultiCurve, egml::io::Error> =
            gml_multi_curve_property.content.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_space.lod2_multi_curve = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod2_multi_curve of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_curve_property) = parsed_result.lod3_multi_curve {
        let multi_curve_result: Result<MultiCurve, egml::io::Error> =
            gml_multi_curve_property.content.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_space.lod3_multi_curve = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod3_multi_curve of feature (id={}) contains invalid geometry: {}",
                    &abstract_space.id(),
                    e.to_string()
                );
            }
        }
    }

    Ok(abstract_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSpace {
    #[serde(rename = "spaceType")]
    pub space_type: Option<GmlSpaceType>,

    #[serde(rename = "lod1Solid")]
    pub lod1_solid: Option<GmlSolidProperty>,
    #[serde(rename = "lod2Solid")]
    pub lod2_solid: Option<GmlSolidProperty>,
    #[serde(rename = "lod3Solid")]
    pub lod3_solid: Option<GmlSolidProperty>,

    #[serde(rename = "lod0MultiSurface")]
    pub lod0_multi_surface: Option<GmlMultiSurfaceProperty>,
    #[serde(rename = "lod2MultiSurface")]
    pub lod2_multi_surface: Option<GmlMultiSurfaceProperty>,
    #[serde(rename = "lod3MultiSurface")]
    pub lod3_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod0MultiCurve")]
    pub lod0_multi_curve: Option<GmlMultiCurveProperty>,
    #[serde(rename = "lod2MultiCurve")]
    pub lod2_multi_curve: Option<GmlMultiCurveProperty>,
    #[serde(rename = "lod3MultiCurve")]
    pub lod3_multi_curve: Option<GmlMultiCurveProperty>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, AsAbstractThematicSurface,
        SpaceType,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_parse_abstract_space_with_solid_with_links() {
        let xml_document = b"\
    <bldg:Building gml:id=\"UUID_d281adfc-4901-0f52-540b-4cc1a9325f82\">
      <lod2Solid>
        <gml:Solid>
          <gml:exterior>
            <gml:Shell>
              <gml:surfaceMember xlink:href=\"#PolyID7350_878_759628_120742\"/>
              <gml:surfaceMember xlink:href=\"#PolyID7351_1722_416019_316876\"/>
              <gml:surfaceMember xlink:href=\"#PolyID7352_230_209861_355851\"/>
              <gml:surfaceMember xlink:href=\"#PolyID7353_166_774155_320806\"/>
              <gml:surfaceMember xlink:href=\"#PolyID7354_1362_450904_410226\"/>
              <gml:surfaceMember xlink:href=\"#PolyID7355_537_416207_260034\"/>
              <gml:surfaceMember xlink:href=\"#PolyID7356_612_880782_415367\"/>
            </gml:Shell>
          </gml:exterior>
        </gml:Solid>
      </lod2Solid>
    </bldg:Building>
";

        let abstract_space = parse_abstract_space(xml_document).expect("should work");

        assert!(abstract_space.lod1_solid().is_none());
        assert!(abstract_space.lod2_solid().is_none()); // TODO: XLink not yet supported
        assert!(abstract_space.lod3_solid().is_none());
    }

    #[test]
    fn test_parse_basic_abstract_space() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
              <genericAttribute>
                <gen:IntAttribute>
                  <gen:name>identifier_laneId</gen:name>
                  <gen:value>1</gen:value>
                </gen:IntAttribute>
              </genericAttribute>
              <genericAttribute>
                <gen:IntAttribute>
                  <gen:name>identifier_laneSectionId</gen:name>
                  <gen:value>0</gen:value>
                </gen:IntAttribute>
              </genericAttribute>
              <spaceType>open</spaceType>
              <lod2Solid>
                <gml:Solid>
                  <gml:exterior>
                    <gml:Shell>
                      <gml:surfaceMember>
                        <gml:Polygon>
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">-49.60668801898414 3.9012103934480056 0.05 -17.879382466595356 28.260563471585364 0.05 -15.44344715878162 25.087832916346485 0.45 -47.1707527111704 0.7284798382091271 0.45 -49.60668801898414 3.9012103934480056 0.05</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon>
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">-47.1707527111704 0.7284798382091271 4.95 -15.44344715878162 25.087832916346485 4.95 -17.879382466595356 28.260563471585364 4.55 -49.60668801898414 3.9012103934480056 4.55 -47.1707527111704 0.7284798382091271 4.95</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon>
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">-49.60668801898414 3.9012103934480056 4.55 -17.879382466595356 28.260563471585364 4.55 -17.879382466595356 28.260563471585364 0.05 -49.60668801898414 3.9012103934480056 0.05 -49.60668801898414 3.9012103934480056 4.55</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon>
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">-47.1707527111704 0.7284798382091271 0.45 -15.44344715878162 25.087832916346485 0.45 -15.44344715878162 25.087832916346485 4.95 -47.1707527111704 0.7284798382091271 4.95 -47.1707527111704 0.7284798382091271 0.45</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon>
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">-49.60668801898414 3.9012103934480056 0.05 -47.1707527111704 0.7284798382091271 0.45 -47.1707527111704 0.7284798382091271 4.95 -49.60668801898414 3.9012103934480056 4.55 -49.60668801898414 3.9012103934480056 0.05</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon>
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">-17.879382466595356 28.260563471585364 0.05 -17.879382466595356 28.260563471585364 4.55 -15.44344715878162 25.087832916346485 4.95 -15.44344715878162 25.087832916346485 0.45 -17.879382466595356 28.260563471585364 0.05</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                    </gml:Shell>
                  </gml:exterior>
                </gml:Solid>
              </lod2Solid>
              <lod2MultiCurve>
                <gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">-48.38872036507727 2.3148451158285663 0.25 -16.661414812688488 26.674198193965925 0.25</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>
              </lod2MultiCurve>
              <tran:function>1</tran:function>
              <tran:usage>2</tran:usage>
              <tran:granularity>lane</tran:granularity>
              <tran:trafficDirection>backwards</tran:trafficDirection>
            </tran:TrafficSpace>";

        let abstract_space = parse_abstract_space(xml_document).expect("should work");

        assert!(abstract_space.lod1_solid().is_none());
        assert!(abstract_space.lod2_solid().is_some());
        assert!(abstract_space.lod2_multi_curve().is_some());
        assert_eq!(abstract_space.space_type(), Some(&SpaceType::Open));
    }
}
