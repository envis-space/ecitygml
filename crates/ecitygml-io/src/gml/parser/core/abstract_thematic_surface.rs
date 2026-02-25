use crate::Error;
use crate::gml::parser::core::parse_abstract_city_object;
use ecitygml_core::model::core::{AbstractThematicSurface, AsAbstractFeature};
use egml::io::aggregates::GmlMultiSurfaceProperty;
use egml::model::geometry::aggregates::MultiSurface;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn parse_abstract_thematic_surface(
    xml_document: &[u8],
) -> Result<AbstractThematicSurface, Error> {
    let abstract_city_object = parse_abstract_city_object(xml_document)?;
    let mut abstract_thematic_surface = AbstractThematicSurface::new(abstract_city_object);
    let parsed_result: GmlAbstractThematicSurface = de::from_reader(xml_document)?;

    if let Some(gml_multi_surface_property) = parsed_result.lod0_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_thematic_surface.lod0_multi_surface = Some(x);
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

    if let Some(gml_multi_surface_property) = parsed_result.lod1_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_thematic_surface.lod1_multi_surface = Some(x);
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

    if let Some(gml_multi_surface_property) = parsed_result.lod2_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_thematic_surface.lod2_multi_surface = Some(x);
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

    if let Some(gml_multi_surface_property) = parsed_result.lod3_multi_surface {
        let multi_surface_result: Result<MultiSurface, egml::io::Error> =
            gml_multi_surface_property.content.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_thematic_surface.lod3_multi_surface = Some(x);
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

    Ok(abstract_thematic_surface)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractThematicSurface {
    #[serde(rename = "lod0MultiSurface")]
    pub lod0_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod1MultiSurface")]
    pub lod1_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod2MultiSurface")]
    pub lod2_multi_surface: Option<GmlMultiSurfaceProperty>,

    #[serde(rename = "lod3MultiSurface")]
    pub lod3_multi_surface: Option<GmlMultiSurfaceProperty>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_parse_basic_abstract_thematic_surface() {
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

        let abstract_thematic_surface =
            parse_abstract_thematic_surface(xml_document).expect("should work");

        assert_eq!(
            abstract_thematic_surface.id(),
            &Id::try_from("test-id-123").expect("should work")
        );
        assert!(abstract_thematic_surface.lod2_multi_surface().is_some());
        assert_eq!(abstract_thematic_surface.generic_attributes().len(), 0);
    }
}
