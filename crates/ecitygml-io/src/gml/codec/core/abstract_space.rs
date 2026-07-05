use crate::error::Error;
use crate::gml::codec::core::enums::GmlSpaceType;
use crate::gml::codec::core::space_boundary_property::{
    deserialize_space_boundary_property, serialize_space_boundary_property,
};
use crate::gml::codec::core::{deserialize_abstract_city_object, serialize_abstract_city_object};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{
    XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_children, serialize_inner,
};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::{
    AbstractSpace, AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, AsAbstractSpaceMut,
};
use egml::io::aggregates::{GmlMultiCurveProperty, GmlMultiSurfaceProperty};
use egml::io::primitives::GmlSolidProperty;
use egml::model::geometry::aggregates::{MultiCurveProperty, MultiSurfaceProperty};
use egml::model::geometry::primitives::SolidProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn deserialize_abstract_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractSpace, Error> {
    let mut abstract_city_object_result = None;
    let mut parsed_result = None;
    let mut space_boundaries_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_city_object_result =
                Some(deserialize_abstract_city_object(xml_document, spans));
        });
        s.spawn(|_| {
            parsed_result =
                Some(de::from_reader::<_, GmlAbstractSpace>(xml_document).map_err(Error::from));
        });
        s.spawn(|_| {
            space_boundaries_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::BoundaryProperty,
                deserialize_space_boundary_property,
            ));
        });
    });

    let abstract_city_object =
        abstract_city_object_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let space_boundaries =
        space_boundaries_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_space = AbstractSpace::from_abstract_city_object(abstract_city_object);
    abstract_space.set_space_type(parsed.space_type.map(Into::into));

    if let Some(gml_solid_property) = parsed.lod1_solid {
        let gml_solid_result: Result<SolidProperty, egml::io::Error> =
            gml_solid_property.try_into();

        match gml_solid_result {
            Ok(x) => {
                abstract_space.set_lod1_solid(Some(x));
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

    if let Some(gml_solid_property) = parsed.lod2_solid {
        let gml_solid_result: Result<SolidProperty, egml::io::Error> =
            gml_solid_property.try_into();

        match gml_solid_result {
            Ok(x) => {
                abstract_space.set_lod2_solid(Some(x));
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

    if let Some(gml_solid_property) = parsed.lod3_solid {
        let gml_solid_result: Result<SolidProperty, egml::io::Error> =
            gml_solid_property.try_into();

        match gml_solid_result {
            Ok(x) => {
                abstract_space.set_lod3_solid(Some(x));
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

    if let Some(gml_multi_surface_property) = parsed.lod0_multi_surface {
        let multi_surface_result: Result<MultiSurfaceProperty, egml::io::Error> =
            gml_multi_surface_property.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_space.set_lod0_multi_surface(Some(x));
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

    if let Some(gml_multi_surface_property) = parsed.lod2_multi_surface {
        let multi_surface_result: Result<MultiSurfaceProperty, egml::io::Error> =
            gml_multi_surface_property.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_space.set_lod2_multi_surface(Some(x));
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

    if let Some(gml_multi_surface_property) = parsed.lod3_multi_surface {
        let multi_surface_result: Result<MultiSurfaceProperty, egml::io::Error> =
            gml_multi_surface_property.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_space.set_lod3_multi_surface(Some(x));
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

    if let Some(gml_multi_curve_property) = parsed.lod0_multi_curve {
        let multi_curve_result: Result<MultiCurveProperty, egml::io::Error> =
            gml_multi_curve_property.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_space.set_lod0_multi_curve(Some(x));
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

    if let Some(gml_multi_curve_property) = parsed.lod2_multi_curve {
        let multi_curve_result: Result<MultiCurveProperty, egml::io::Error> =
            gml_multi_curve_property.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_space.set_lod2_multi_curve(Some(x));
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

    if let Some(gml_multi_curve_property) = parsed.lod3_multi_curve {
        let multi_curve_result: Result<MultiCurveProperty, egml::io::Error> =
            gml_multi_curve_property.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_space.set_lod3_multi_curve(Some(x));
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

    abstract_space.set_boundaries(space_boundaries);

    Ok(abstract_space)
}

pub fn serialize_abstract_space(
    abstract_space: &AbstractSpace,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts =
        serialize_abstract_city_object(abstract_space.abstract_city_object(), formatting)?;

    if let Some(raw) = serialize_inner(GmlAbstractSpace::from(abstract_space), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    xml_node_parts.content.extend(
        abstract_space
            .boundaries()
            .iter()
            .map(|x| serialize_space_boundary_property(x, formatting).map(XmlNodeContent::from))
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSpace {
    #[serde(rename = "spaceType", skip_serializing_if = "Option::is_none")]
    pub space_type: Option<GmlSpaceType>,

    #[serde(rename = "lod1Solid", skip_serializing_if = "Option::is_none")]
    pub lod1_solid: Option<GmlSolidProperty>,
    #[serde(rename = "lod2Solid", skip_serializing_if = "Option::is_none")]
    pub lod2_solid: Option<GmlSolidProperty>,
    #[serde(rename = "lod3Solid", skip_serializing_if = "Option::is_none")]
    pub lod3_solid: Option<GmlSolidProperty>,

    #[serde(rename = "lod0MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod0_multi_surface: Option<GmlMultiSurfaceProperty>,
    #[serde(rename = "lod2MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod2_multi_surface: Option<GmlMultiSurfaceProperty>,
    #[serde(rename = "lod3MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod3_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod0MultiCurve", skip_serializing_if = "Option::is_none")]
    pub lod0_multi_curve: Option<GmlMultiCurveProperty>,
    #[serde(rename = "lod2MultiCurve", skip_serializing_if = "Option::is_none")]
    pub lod2_multi_curve: Option<GmlMultiCurveProperty>,
    #[serde(rename = "lod3MultiCurve", skip_serializing_if = "Option::is_none")]
    pub lod3_multi_curve: Option<GmlMultiCurveProperty>,
}

impl From<&AbstractSpace> for GmlAbstractSpace {
    fn from(item: &AbstractSpace) -> Self {
        Self {
            space_type: item.space_type().map(Into::into),
            lod1_solid: item.lod1_solid().map(Into::into),
            lod2_solid: item.lod2_solid().map(Into::into),
            lod3_solid: item.lod3_solid().map(Into::into),
            lod0_multi_surface: item.lod0_multi_surface().map(Into::into),
            lod2_multi_surface: item.lod2_multi_surface().map(Into::into),
            lod3_multi_surface: item.lod3_multi_surface().map(Into::into),
            lod0_multi_curve: item.lod0_multi_curve().map(Into::into),
            lod2_multi_curve: item.lod2_multi_curve().map(Into::into),
            lod3_multi_curve: item.lod3_multi_curve().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::util::extract_xml_element_spans;
    use ecitygml_core::model::core::AsAbstractSpace;
    use ecitygml_core::model::core::enums::SpaceType;

    #[test]
    fn test_deserialize_abstract_space_with_solid_with_links() {
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        assert!(abstract_space.lod1_solid().is_none());
        assert!(abstract_space.lod2_solid().is_some());
        assert!(abstract_space.lod3_solid().is_none());

        let lod2_solid = abstract_space.lod2_solid().expect("should work");
        let exterior_shell = lod2_solid
            .object
            .as_ref()
            .unwrap()
            .exterior()
            .as_ref()
            .unwrap()
            .object
            .as_ref()
            .unwrap();
        assert_eq!(exterior_shell.members().len(), 7);
    }

    #[test]
    fn test_deserialize_basic_abstract_space() {
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        assert!(abstract_space.lod1_solid().is_none());
        assert!(abstract_space.lod2_solid().is_some());
        assert!(abstract_space.lod2_multi_curve().is_some());
        assert_eq!(abstract_space.space_type(), Some(SpaceType::Open));
    }
}
