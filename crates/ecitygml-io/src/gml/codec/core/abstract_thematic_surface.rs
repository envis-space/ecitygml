use crate::Error;
use crate::gml::codec::core::point_cloud_property::{
    deserialize_point_cloud_property, serialize_point_cloud_property,
};
use crate::gml::codec::core::{
    deserialize_abstract_space_boundary, serialize_abstract_space_boundary,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNodeContent, XmlNodeParts};
use crate::gml::util::{collect_child, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::{
    AbstractThematicSurface, AsAbstractFeature, AsAbstractSpaceBoundary, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut,
};
use egml::io::aggregates::{GmlMultiCurveProperty, GmlMultiSurfaceProperty};
use egml::model::geometry::aggregates::{MultiCurveProperty, MultiSurfaceProperty};
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn deserialize_abstract_thematic_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractThematicSurface, Error> {
    let mut abstract_space_boundary_result = None;
    let mut parsed_result = None;
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
            point_cloud_result = Some(collect_child(
                xml_document,
                spans,
                XmlElement::PointCloudProperty,
                deserialize_point_cloud_property,
            ));
        });
    });

    let abstract_space_boundary =
        abstract_space_boundary_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let point_cloud = point_cloud_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_thematic_surface =
        AbstractThematicSurface::from_abstract_space_boundary(abstract_space_boundary);

    if let Some(gml_multi_surface_property) = parsed.lod0_multi_surface {
        let multi_surface_result: Result<MultiSurfaceProperty, egml::io::Error> =
            gml_multi_surface_property.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_thematic_surface.set_lod0_multi_surface(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod0_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_thematic_surface.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_surface_property) = parsed.lod1_multi_surface {
        let multi_surface_result: Result<MultiSurfaceProperty, egml::io::Error> =
            gml_multi_surface_property.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_thematic_surface.set_lod1_multi_surface(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod1_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_thematic_surface.id(),
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
                abstract_thematic_surface.set_lod2_multi_surface(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod2_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_thematic_surface.id(),
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
                abstract_thematic_surface.set_lod3_multi_surface(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod3_multi_surface of feature (id={}) contains invalid geometry: {}",
                    &abstract_thematic_surface.id(),
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
                abstract_thematic_surface.set_lod0_multi_curve(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod0_multi_curve of feature (id={}) contains invalid geometry: {}",
                    &abstract_thematic_surface.id(),
                    e.to_string()
                );
            }
        }
    }

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

    xml_node_parts.content.extend(
        abstract_thematic_surface
            .point_cloud()
            .iter()
            .map(|x| serialize_point_cloud_property(x, formatting).map(XmlNodeContent::from))
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractThematicSurface {
    #[serde(rename = "lod0MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod0_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod1MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod1_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod2MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod2_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod3MultiSurface", skip_serializing_if = "Option::is_none")]
    pub lod3_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod0MultiCurve", skip_serializing_if = "Option::is_none")]
    pub lod0_multi_curve: Option<GmlMultiCurveProperty>,
}

impl From<&AbstractThematicSurface> for GmlAbstractThematicSurface {
    fn from(item: &AbstractThematicSurface) -> Self {
        Self {
            lod0_multi_surface: item.lod0_multi_surface().map(Into::into),
            lod1_multi_surface: item.lod1_multi_surface().map(Into::into),
            lod2_multi_surface: item.lod2_multi_surface().map(Into::into),
            lod3_multi_surface: item.lod3_multi_surface().map(Into::into),
            lod0_multi_curve: item.lod0_multi_curve().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::util::extract_xml_element_spans;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

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
            abstract_thematic_surface.id(),
            &Id::try_from("test-id-123").expect("should work")
        );
        assert!(abstract_thematic_surface.lod2_multi_surface().is_some());
        assert_eq!(abstract_thematic_surface.generic_attributes().len(), 0);
    }
}
