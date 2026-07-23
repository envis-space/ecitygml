use crate::Error;
use crate::gml::codec::core::abstract_point_cloud_property::{
    deserialize_abstract_point_cloud_property, serialize_abstract_point_cloud_property,
};
use crate::gml::codec::core::qualified_area_property::GmlAreaProperty;
use crate::gml::codec::core::{
    deserialize_abstract_space_boundary, serialize_abstract_space_boundary,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement, collect_gml_child_lenient};
use ecitygml_core::model::core::{
    AbstractThematicSurface, AsAbstractSpaceBoundary, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut,
};
use egml::io::codec::geometry::aggregates::{
    deserialize_multi_curve_property, deserialize_multi_surface_property,
    serialize_multi_curve_property, serialize_multi_surface_property,
};
use egml::io::util::{
    Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_child, serialize_inner,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_thematic_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractThematicSurface, Error> {
    let mut abstract_space_boundary_result = None;
    let mut parsed_result = None;
    let mut lod0_multi_surface_result = None;
    let mut lod1_multi_surface_result = None;
    let mut lod2_multi_surface_result = None;
    let mut lod3_multi_surface_result = None;
    let mut lod0_multi_curve_result = None;
    let mut point_cloud_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_space_boundary_result =
                Some(deserialize_abstract_space_boundary(xml_document, spans));
        });
        s.spawn(|_| {
            parsed_result = Some(
                de::from_reader::<_, GmlAbstractThematicSurface>(xml_document).map_err(Error::from),
            );
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
            lod1_multi_surface_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod1MultiSurfaceProperty.into(),
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
            point_cloud_result = Some(collect_child(
                xml_document,
                spans,
                CityGmlElement::AbstractPointCloudProperty.into(),
                deserialize_abstract_point_cloud_property,
            ));
        });
    });

    let abstract_space_boundary =
        abstract_space_boundary_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let lod0_multi_surface =
        lod0_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod1_multi_surface =
        lod1_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod2_multi_surface =
        lod2_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod3_multi_surface =
        lod3_multi_surface_result.expect("rayon::scope guarantees all spawns complete");
    let lod0_multi_curve =
        lod0_multi_curve_result.expect("rayon::scope guarantees all spawns complete");
    let point_cloud = point_cloud_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_thematic_surface =
        AbstractThematicSurface::from_abstract_space_boundary(abstract_space_boundary);
    abstract_thematic_surface.set_areas(parsed.areas.into_iter().map(Into::into).collect());
    abstract_thematic_surface.set_lod0_multi_surface(lod0_multi_surface);
    abstract_thematic_surface.set_lod1_multi_surface(lod1_multi_surface);
    abstract_thematic_surface.set_lod2_multi_surface(lod2_multi_surface);
    abstract_thematic_surface.set_lod3_multi_surface(lod3_multi_surface);
    abstract_thematic_surface.set_lod0_multi_curve(lod0_multi_curve);
    abstract_thematic_surface.set_point_cloud(point_cloud);

    Ok(abstract_thematic_surface)
}

pub fn serialize_abstract_thematic_surface(
    abstract_thematic_surface: &AbstractThematicSurface,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_space_boundary(
        abstract_thematic_surface.abstract_space_boundary(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractThematicSurface::from(abstract_thematic_surface),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    if let Some(prop) = abstract_thematic_surface.lod0_multi_surface() {
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
    if let Some(prop) = abstract_thematic_surface.lod1_multi_surface() {
        xml_node_parts.content.push(
            serialize_multi_surface_property(
                prop,
                formatting,
                CityGmlElement::Lod1MultiSurfaceProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_thematic_surface.lod2_multi_surface() {
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
    if let Some(prop) = abstract_thematic_surface.lod3_multi_surface() {
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
    if let Some(prop) = abstract_thematic_surface.lod0_multi_curve() {
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
    xml_node_parts.content.extend(
        abstract_thematic_surface
            .point_cloud()
            .iter()
            .map(|x| {
                serialize_abstract_point_cloud_property(x, formatting).map(XmlNodeContent::from)
            })
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractThematicSurface {
    #[serde(rename = "area", default)]
    pub areas: Vec<GmlAreaProperty>,
}

impl From<&AbstractThematicSurface> for GmlAbstractThematicSurface {
    fn from(item: &AbstractThematicSurface) -> Self {
        Self {
            areas: item.areas().iter().map(GmlAreaProperty::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::io::util::extract_xml_element_spans;
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_with_lod2_multi_surface() {
        let xml_document = b"\
<con:WallSurface gml:id=\"test-id-123\">
  <gml:name>Outer Wall 1 (West)</gml:name>
  <lod2MultiSurface>
    <gml:MultiSurface>
      <gml:surfaceMember>
        <gml:Polygon gml:id=\"PolyID7350_878_759628_120742\">
          <gml:exterior>
            <gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
              <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
              <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
              <gml:pos>457842.0 5439093.0 111.8</gml:pos>
              <gml:pos>457842.0 5439083.0 111.8</gml:pos>
              <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
              <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
            </gml:LinearRing>
          </gml:exterior>
        </gml:Polygon>
      </gml:surfaceMember>
    </gml:MultiSurface>
  </lod2MultiSurface>
</con:WallSurface>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_thematic_surface =
            deserialize_abstract_thematic_surface(xml_document, &spans).expect("should work");

        assert_eq!(
            abstract_thematic_surface.feature_id(),
            &Id::try_from("test-id-123").expect("should work")
        );
        assert!(abstract_thematic_surface.lod2_multi_surface().is_some());
        assert_eq!(abstract_thematic_surface.generic_attributes().len(), 0);
    }

    #[test]
    fn test_deserialize_abstract_thematic_surface_with_area() {
        let xml_document = b"\
<con:WallSurface gml:id=\"test-id-123\">
  <area>
    <QualifiedArea>
      <area uom=\"m2\">5.0</area>
      <typeOfArea>RoadArea</typeOfArea>
    </QualifiedArea>
  </area>
</con:WallSurface>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_thematic_surface =
            deserialize_abstract_thematic_surface(xml_document, &spans).expect("should work");

        assert_eq!(abstract_thematic_surface.areas().len(), 1);

        let area = &abstract_thematic_surface.areas()[0];
        assert_eq!(area.area().value(), 5.0);
        assert_eq!(area.area().uom(), "m2");
        assert_eq!(area.type_of_area().code().value(), "RoadArea");
    }

    #[test]
    fn test_serialize_abstract_thematic_surface_with_area() {
        let xml_document = b"\
<con:WallSurface gml:id=\"test-id-123\">
  <area>
    <QualifiedArea>
      <area uom=\"m2\">5.0</area>
      <typeOfArea>WallAreaMeasured</typeOfArea>
    </QualifiedArea>
  </area>
</con:WallSurface>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_thematic_surface =
            deserialize_abstract_thematic_surface(xml_document, &spans).expect("should work");

        let xml_node_parts =
            serialize_abstract_thematic_surface(&abstract_thematic_surface, Formatting::Compact)
                .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("con:WallSurface", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("<area>") || xml.contains("<area "));
        assert!(xml.contains("QualifiedArea"));
        assert!(xml.contains("uom=\"m2\""));
        assert!(xml.contains('5'));
        assert!(xml.contains("typeOfArea"));
        assert!(xml.contains("WallAreaMeasured"));
    }

    #[test]
    fn test_round_trip_abstract_thematic_surface_with_area() {
        let xml_document = b"\
<con:WallSurface gml:id=\"test-id-123\">
  <area>
    <QualifiedArea>
      <area uom=\"m2\">5.0</area>
      <typeOfArea>WallAreaMeasured</typeOfArea>
    </QualifiedArea>
  </area>
</con:WallSurface>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_thematic_surface =
            deserialize_abstract_thematic_surface(xml_document, &spans).expect("should work");

        let xml_node_parts =
            serialize_abstract_thematic_surface(&abstract_thematic_surface, Formatting::Compact)
                .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("con:WallSurface", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        let round_tripped_spans = extract_xml_element_spans(xml.as_bytes()).expect("should work");
        let round_tripped =
            deserialize_abstract_thematic_surface(xml.as_bytes(), &round_tripped_spans)
                .expect("should work");

        assert_eq!(abstract_thematic_surface.areas(), round_tripped.areas());
    }
}
