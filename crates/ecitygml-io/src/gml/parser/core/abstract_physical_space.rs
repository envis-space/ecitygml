use crate::Error;
use crate::gml::parser::core::deserialize_abstract_space;
use ecitygml_core::model::core::{
    AbstractPhysicalSpace, AsAbstractFeature, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut,
};
use egml::io::aggregates::GmlMultiCurveProperty;
use egml::model::geometry::aggregates::MultiCurve;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn deserialize_abstract_physical_space(
    xml_document: &[u8],
) -> Result<AbstractPhysicalSpace, Error> {
    let abstract_space = deserialize_abstract_space(xml_document)?;
    let parsed_result: GmlAbstractPhysicalSpace = de::from_reader(xml_document)?;
    let mut abstract_physical_space = AbstractPhysicalSpace::new(abstract_space);

    if let Some(gml_multi_curve_property) = parsed_result.lod1_terrain_intersection_curve {
        let multi_curve_result: Result<MultiCurve, egml::io::Error> =
            gml_multi_curve_property.content.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_physical_space.set_lod1_terrain_intersection_curve(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod1TerrainIntersectionCurve of feature (id={}) contains invalid geometry: {}",
                    &abstract_physical_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_curve_property) = parsed_result.lod2_terrain_intersection_curve {
        let multi_curve_result: Result<MultiCurve, egml::io::Error> =
            gml_multi_curve_property.content.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_physical_space.set_lod2_terrain_intersection_curve(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod2TerrainIntersectionCurve of feature (id={}) contains invalid geometry: {}",
                    &abstract_physical_space.id(),
                    e.to_string()
                );
            }
        }
    }

    if let Some(gml_multi_curve_property) = parsed_result.lod3_terrain_intersection_curve {
        let multi_curve_result: Result<MultiCurve, egml::io::Error> =
            gml_multi_curve_property.content.try_into();

        match multi_curve_result {
            Ok(x) => {
                abstract_physical_space.set_lod3_terrain_intersection_curve(Some(x));
            }
            Err(e) => {
                debug!(
                    "lod3TerrainIntersectionCurve of feature (id={}) contains invalid geometry: {}",
                    &abstract_physical_space.id(),
                    e.to_string()
                );
            }
        }
    }

    Ok(abstract_physical_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractPhysicalSpace {
    #[serde(rename = "lod1TerrainIntersectionCurve")]
    pub lod1_terrain_intersection_curve: Option<GmlMultiCurveProperty>,

    #[serde(rename = "lod2TerrainIntersectionCurve")]
    pub lod2_terrain_intersection_curve: Option<GmlMultiCurveProperty>,

    #[serde(rename = "lod3TerrainIntersectionCurve")]
    pub lod3_terrain_intersection_curve: Option<GmlMultiCurveProperty>,
}
