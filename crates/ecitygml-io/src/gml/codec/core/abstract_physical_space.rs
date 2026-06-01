use crate::Error;
use crate::gml::codec::core::deserialize_abstract_space;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::{
    AbstractPhysicalSpace, AsAbstractFeature, AsAbstractPhysicalSpaceMut,
};
use egml::io::aggregates::GmlMultiCurveProperty;
use egml::model::geometry::aggregates::MultiCurve;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn deserialize_abstract_physical_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractPhysicalSpace, Error> {
    let (abstract_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_space(xml_document, spans),
        || de::from_reader::<_, GmlAbstractPhysicalSpace>(xml_document).map_err(Error::from),
    );
    let abstract_space = abstract_space_result?;
    let parsed = parsed_result?;
    let mut abstract_physical_space = AbstractPhysicalSpace::new(abstract_space);

    if let Some(gml_multi_curve_property) = parsed.lod1_terrain_intersection_curve {
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

    if let Some(gml_multi_curve_property) = parsed.lod2_terrain_intersection_curve {
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

    if let Some(gml_multi_curve_property) = parsed.lod3_terrain_intersection_curve {
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
