use crate::Error;
use crate::gml::parser::core::implicit_geometry_property::GmlImplicitGeometryProperty;
use crate::gml::parser::core::parse_abstract_space;
use ecitygml_core::model::core::{AbstractOccupiedSpace, AsAbstractFeature, ImplicitGeometry};
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn parse_abstract_occupied_space(xml_document: &[u8]) -> Result<AbstractOccupiedSpace, Error> {
    let abstract_space = parse_abstract_space(xml_document)?;
    let mut abstract_occupied_space = AbstractOccupiedSpace::new(abstract_space);
    let parsed_result: GmlAbstractOccupiedSpace = de::from_reader(xml_document)?;

    if let Some(gml_implicit_geometry_property) = parsed_result.lod1_implicit_representation {
        let multi_surface_result: Result<ImplicitGeometry, Error> =
            gml_implicit_geometry_property.implicit_geometry.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_occupied_space.lod1_implicit_representation = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod1ImplicitRepresentation of feature (id={}) contains invalid geometry: {}",
                    &abstract_occupied_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_implicit_geometry_property) = parsed_result.lod2_implicit_representation {
        let multi_surface_result: Result<ImplicitGeometry, Error> =
            gml_implicit_geometry_property.implicit_geometry.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_occupied_space.lod2_implicit_representation = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod2ImplicitRepresentation of feature (id={}) contains invalid geometry: {}",
                    &abstract_occupied_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_implicit_geometry_property) = parsed_result.lod3_implicit_representation {
        let multi_surface_result: Result<ImplicitGeometry, Error> =
            gml_implicit_geometry_property.implicit_geometry.try_into();

        match multi_surface_result {
            Ok(x) => {
                abstract_occupied_space.lod3_implicit_representation = Some(x);
            }
            Err(e) => {
                debug!(
                    "lod3ImplicitRepresentation of feature (id={}) contains invalid geometry: {}",
                    &abstract_occupied_space.id(),
                    e.to_string()
                );
            }
        }
    }

    Ok(abstract_occupied_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractOccupiedSpace {
    #[serde(rename = "lod1ImplicitRepresentation")]
    pub lod1_implicit_representation: Option<GmlImplicitGeometryProperty>,

    #[serde(rename = "lod2ImplicitRepresentation")]
    pub lod2_implicit_representation: Option<GmlImplicitGeometryProperty>,

    #[serde(rename = "lod3ImplicitRepresentation")]
    pub lod3_implicit_representation: Option<GmlImplicitGeometryProperty>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractOccupiedSpace;

    #[test]
    fn test_parse_abstract_occupied_space() {
        let xml_document = b"<frn:CityFurniture gml:id=\"UUID_35904d1c-ecd3-30d8-b8a5-02710feebc2d\">
      <gml:name>Private_Parking</gml:name>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>identifier_roadObjectName</gen:name>
          <gen:value>Private_Parking</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <lod2MultiSurface>
        <gml:MultiSurface>
          <gml:surfaceMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList srsDimension=\"3\">678279.7493302015 5403792.6862397045 369.4773337509289 678279.7493302015 5403792.6862397045 369.2473337509289 678279.450359054 5403792.98122488 369.2473337509289 678279.450359054 5403792.98122488 369.4773337509289 678279.7493302015 5403792.6862397045 369.4773337509289</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:surfaceMember>
        </gml:MultiSurface>
      </lod2MultiSurface>
      <lod1ImplicitRepresentation>
        <ImplicitGeometry>
          <transformationMatrix>-0.7023456529181015 0.7118360652755983 0.0 0.0 -0.7118360652755983 -0.7023456529181015 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
          <referencePoint>
            <gml:Point>
              <gml:pos srsDimension=\"3\">678279.5998446278 5403792.833732292 369.2473337509289</gml:pos>
            </gml:Point>
          </referencePoint>
        </ImplicitGeometry>
      </lod1ImplicitRepresentation>
    </frn:CityFurniture>";

        let abstract_occupied_space =
            parse_abstract_occupied_space(xml_document).expect("should work");

        assert!(
            abstract_occupied_space
                .lod1_implicit_representation()
                .is_some()
        );
        assert!(
            abstract_occupied_space
                .lod2_implicit_representation()
                .is_none()
        );
        assert!(
            abstract_occupied_space
                .lod3_implicit_representation()
                .is_none()
        );

        let lod1_implicit_representation = abstract_occupied_space
            .lod1_implicit_representation()
            .unwrap();
        assert_eq!(
            lod1_implicit_representation.reference_point.pos().x(),
            678279.5998446278
        );
        assert_eq!(
            lod1_implicit_representation.reference_point.pos().y(),
            5403792.833732292
        );
        assert_eq!(
            lod1_implicit_representation.reference_point.pos().z(),
            369.2473337509289
        );
    }
}
