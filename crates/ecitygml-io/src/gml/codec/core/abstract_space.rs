use crate::error::Error;
use crate::gml::codec::core::abstract_space_boundary_property::{
    deserialize_abstract_space_boundary_property, serialize_abstract_space_boundary_property,
};
use crate::gml::codec::core::enums::GmlSpaceType;
use crate::gml::codec::core::qualified_area_property::GmlAreaProperty;
use crate::gml::codec::core::qualified_volume_property::GmlVolumeProperty;
use crate::gml::codec::core::{deserialize_abstract_city_object, serialize_abstract_city_object};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement, collect_gml_child_lenient};
use ecitygml_core::model::core::{
    AbstractSpace, AsAbstractCityObject, AsAbstractSpace, AsAbstractSpaceMut,
};
use egml::io::codec::geometry::aggregates::{
    deserialize_multi_curve_property, deserialize_multi_surface_property,
    serialize_multi_curve_property, serialize_multi_surface_property,
};
use egml::io::codec::geometry::primitives::{deserialize_solid_property, serialize_solid_property};
use egml::io::util::collect_children;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_space(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractSpace, Error> {
    let mut abstract_city_object_result = None;
    let mut parsed_result = None;
    let mut space_boundaries_result = None;
    let mut lod1_solid_result = None;
    let mut lod2_solid_result = None;
    let mut lod3_solid_result = None;
    let mut lod0_multi_surface_result = None;
    let mut lod2_multi_surface_result = None;
    let mut lod3_multi_surface_result = None;
    let mut lod0_multi_curve_result = None;
    let mut lod2_multi_curve_result = None;
    let mut lod3_multi_curve_result = None;

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
                CityGmlElement::BoundaryProperty.into(),
                deserialize_abstract_space_boundary_property,
            ));
        });
        s.spawn(|_| {
            lod1_solid_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod1SolidProperty.into(),
                deserialize_solid_property,
            ));
        });
        s.spawn(|_| {
            lod2_solid_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod2SolidProperty.into(),
                deserialize_solid_property,
            ));
        });
        s.spawn(|_| {
            lod3_solid_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod3SolidProperty.into(),
                deserialize_solid_property,
            ));
        });
        s.spawn(|_| {
            lod0_multi_surface_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod0MultiSurfaceProperty.into(),
                deserialize_multi_surface_property,
            ));
        });
        s.spawn(|_| {
            lod2_multi_surface_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod2MultiSurfaceProperty.into(),
                deserialize_multi_surface_property,
            ));
        });
        s.spawn(|_| {
            lod3_multi_surface_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod3MultiSurfaceProperty.into(),
                deserialize_multi_surface_property,
            ));
        });
        s.spawn(|_| {
            lod0_multi_curve_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod0MultiCurveProperty.into(),
                deserialize_multi_curve_property,
            ));
        });
        s.spawn(|_| {
            lod2_multi_curve_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod2MultiCurveProperty.into(),
                deserialize_multi_curve_property,
            ));
        });
        s.spawn(|_| {
            lod3_multi_curve_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod3MultiCurveProperty.into(),
                deserialize_multi_curve_property,
            ));
        });
    });

    let abstract_city_object =
        abstract_city_object_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let space_boundaries =
        space_boundaries_result.expect("rayon::scope guarantees all spawns complete")?;
    let lod1_solid = lod1_solid_result.expect("rayon::scope guarantees all spawns complete");
    let lod2_solid = lod2_solid_result.expect("rayon::scope guarantees all spawns complete");
    let lod3_solid = lod3_solid_result.expect("rayon::scope guarantees all spawns complete");
    let lod0_multi_surface =
        lod0_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod2_multi_surface =
        lod2_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod3_multi_surface =
        lod3_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod0_multi_curve =
        lod0_multi_curve_result.expect("rayon::scope guarantees all spawns complete");
    let lod2_multi_curve =
        lod2_multi_curve_result.expect("rayon::scope guarantees all spawns complete");
    let lod3_multi_curve =
        lod3_multi_curve_result.expect("rayon::scope guarantees all spawns complete");

    let mut abstract_space = AbstractSpace::from_abstract_city_object(abstract_city_object);
    abstract_space.set_space_type(parsed.space_type.map(Into::into));
    abstract_space.set_volumes(parsed.volumes.into_iter().map(Into::into).collect());
    abstract_space.set_areas(parsed.areas.into_iter().map(Into::into).collect());
    abstract_space.set_lod1_solid(lod1_solid);
    abstract_space.set_lod2_solid(lod2_solid);
    abstract_space.set_lod3_solid(lod3_solid);
    abstract_space.set_lod0_multi_surface(lod0_multi_surface);
    abstract_space.set_lod2_multi_surface(lod2_multi_surface);
    abstract_space.set_lod3_multi_surface(lod3_multi_surface);
    abstract_space.set_lod0_multi_curve(lod0_multi_curve);
    abstract_space.set_lod2_multi_curve(lod2_multi_curve);
    abstract_space.set_lod3_multi_curve(lod3_multi_curve);
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

    if let Some(prop) = abstract_space.lod1_solid() {
        xml_node_parts.content.push(
            serialize_solid_property(prop, formatting, CityGmlElement::Lod1SolidProperty.into())
                .map_err(Error::from)?
                .into(),
        );
    }
    if let Some(prop) = abstract_space.lod2_solid() {
        xml_node_parts.content.push(
            serialize_solid_property(prop, formatting, CityGmlElement::Lod2SolidProperty.into())
                .map_err(Error::from)?
                .into(),
        );
    }
    if let Some(prop) = abstract_space.lod3_solid() {
        xml_node_parts.content.push(
            serialize_solid_property(prop, formatting, CityGmlElement::Lod3SolidProperty.into())
                .map_err(Error::from)?
                .into(),
        );
    }
    if let Some(prop) = abstract_space.lod0_multi_surface() {
        xml_node_parts.content.push(
            serialize_multi_surface_property(
                prop,
                formatting,
                CityGmlElement::Lod0MultiSurfaceProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_space.lod2_multi_surface() {
        xml_node_parts.content.push(
            serialize_multi_surface_property(
                prop,
                formatting,
                CityGmlElement::Lod2MultiSurfaceProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_space.lod3_multi_surface() {
        xml_node_parts.content.push(
            serialize_multi_surface_property(
                prop,
                formatting,
                CityGmlElement::Lod3MultiSurfaceProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_space.lod0_multi_curve() {
        xml_node_parts.content.push(
            serialize_multi_curve_property(
                prop,
                formatting,
                CityGmlElement::Lod0MultiCurveProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_space.lod2_multi_curve() {
        xml_node_parts.content.push(
            serialize_multi_curve_property(
                prop,
                formatting,
                CityGmlElement::Lod2MultiCurveProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_space.lod3_multi_curve() {
        xml_node_parts.content.push(
            serialize_multi_curve_property(
                prop,
                formatting,
                CityGmlElement::Lod3MultiCurveProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    xml_node_parts.content.extend(
        abstract_space
            .boundaries()
            .iter()
            .map(|x| {
                serialize_abstract_space_boundary_property(x, formatting).map(XmlNodeContent::from)
            })
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSpace {
    #[serde(rename = "spaceType", skip_serializing_if = "Option::is_none")]
    pub space_type: Option<GmlSpaceType>,

    #[serde(rename = "volume", default)]
    pub volumes: Vec<GmlVolumeProperty>,

    #[serde(rename = "area", default)]
    pub areas: Vec<GmlAreaProperty>,
}

impl From<&AbstractSpace> for GmlAbstractSpace {
    fn from(item: &AbstractSpace) -> Self {
        Self {
            space_type: item.space_type().map(Into::into),
            volumes: item.volumes().iter().map(GmlVolumeProperty::from).collect(),
            areas: item.areas().iter().map(GmlAreaProperty::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractSpace;
    use ecitygml_core::model::core::enums::SpaceType;
    use egml::io::util::extract_xml_element_spans;

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
            .object()
            .unwrap()
            .exterior()
            .as_ref()
            .unwrap()
            .object()
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

    #[test]
    fn test_deserialize_abstract_space_with_volume() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <spaceType>open</spaceType>
    <volume>
        <QualifiedVolume>
            <volume uom=\"m3\">5.0</volume>
            <typeOfVolume>RoadVolume</typeOfVolume>
        </QualifiedVolume>
    </volume>
</tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        assert_eq!(abstract_space.space_type(), Some(SpaceType::Open));
        assert_eq!(abstract_space.volumes().len(), 1);

        let volume = &abstract_space.volumes()[0];
        assert_eq!(volume.volume().value(), 5.0);
        assert_eq!(volume.volume().uom(), "m3");
        assert_eq!(volume.type_of_volume().code().value(), "RoadVolume");
    }

    #[test]
    fn test_serialize_abstract_space_with_volume() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <spaceType>open</spaceType>
    <volume>
        <QualifiedVolume>
            <volume uom=\"m3\">5.0</volume>
            <typeOfVolume>RoadVolume</typeOfVolume>
        </QualifiedVolume>
    </volume>
</tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        let xml_node_parts = serialize_abstract_space(&abstract_space, Formatting::Compact)
            .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("tran:TrafficSpace", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("<volume>") || xml.contains("<volume "));
        assert!(xml.contains("QualifiedVolume"));
        assert!(xml.contains("uom=\"m3\""));
        assert!(xml.contains('5'));
        assert!(xml.contains("typeOfVolume"));
        assert!(xml.contains("RoadVolume"));
    }

    #[test]
    fn test_round_trip_abstract_space_with_volume() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <spaceType>open</spaceType>
    <volume>
        <QualifiedVolume>
            <volume uom=\"m3\">5.0</volume>
            <typeOfVolume>RoadVolume</typeOfVolume>
        </QualifiedVolume>
    </volume>
</tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        let xml_node_parts = serialize_abstract_space(&abstract_space, Formatting::Compact)
            .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("tran:TrafficSpace", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        let round_tripped_spans = extract_xml_element_spans(xml.as_bytes()).expect("should work");
        let round_tripped =
            deserialize_abstract_space(xml.as_bytes(), &round_tripped_spans).expect("should work");

        assert_eq!(abstract_space.volumes(), round_tripped.volumes());
    }

    #[test]
    fn test_deserialize_abstract_space_with_area() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <spaceType>open</spaceType>
    <area>
        <QualifiedArea>
            <area uom=\"m2\">5.0</area>
            <typeOfArea>RoadArea</typeOfArea>
        </QualifiedArea>
    </area>
</tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        assert_eq!(abstract_space.space_type(), Some(SpaceType::Open));
        assert_eq!(abstract_space.areas().len(), 1);

        let area = &abstract_space.areas()[0];
        assert_eq!(area.area().value(), 5.0);
        assert_eq!(area.area().uom(), "m2");
        assert_eq!(area.type_of_area().code().value(), "RoadArea");
    }

    #[test]
    fn test_serialize_abstract_space_with_area() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <spaceType>open</spaceType>
    <area>
        <QualifiedArea>
            <area uom=\"m2\">5.0</area>
            <typeOfArea>RoadArea</typeOfArea>
        </QualifiedArea>
    </area>
</tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        let xml_node_parts = serialize_abstract_space(&abstract_space, Formatting::Compact)
            .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("tran:TrafficSpace", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("<area>") || xml.contains("<area "));
        assert!(xml.contains("QualifiedArea"));
        assert!(xml.contains("uom=\"m2\""));
        assert!(xml.contains('5'));
        assert!(xml.contains("typeOfArea"));
        assert!(xml.contains("RoadArea"));
    }

    #[test]
    fn test_round_trip_abstract_space_with_area() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <spaceType>open</spaceType>
    <area>
        <QualifiedArea>
            <area uom=\"m2\">5.0</area>
            <typeOfArea>RoadArea</typeOfArea>
        </QualifiedArea>
    </area>
</tran:TrafficSpace>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_space = deserialize_abstract_space(xml_document, &spans).expect("should work");

        let xml_node_parts = serialize_abstract_space(&abstract_space, Formatting::Compact)
            .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("tran:TrafficSpace", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        let round_tripped_spans = extract_xml_element_spans(xml.as_bytes()).expect("should work");
        let round_tripped =
            deserialize_abstract_space(xml.as_bytes(), &round_tripped_spans).expect("should work");

        assert_eq!(abstract_space.areas(), round_tripped.areas());
    }
}
