use crate::Error;
use crate::gml::codec::core::point_cloud_property::{
    deserialize_point_cloud_property, serialize_point_cloud_property,
};
use crate::gml::codec::core::{deserialize_abstract_space, serialize_abstract_space};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{
    XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_child, serialize_inner,
};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::{
    AbstractPhysicalSpace, AsAbstractFeature, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut,
    AsAbstractSpace,
};
use egml::io::aggregates::GmlMultiCurveProperty;
use egml::model::geometry::aggregates::MultiCurveProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn deserialize_abstract_physical_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractPhysicalSpace, Error> {
    let mut abstract_space_result = None;
    let mut parsed_result = None;
    let mut point_cloud_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_space_result = Some(deserialize_abstract_space(xml_document, spans));
        });
        s.spawn(|_| {
            parsed_result = Some(
                de::from_reader::<_, GmlAbstractPhysicalSpace>(xml_document).map_err(Error::from),
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

    let abstract_space =
        abstract_space_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let point_cloud = point_cloud_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_physical_space = AbstractPhysicalSpace::from_abstract_space(abstract_space);

    if let Some(gml_multi_curve_property) = parsed.lod1_terrain_intersection_curve {
        let multi_curve_result: Result<MultiCurveProperty, egml::io::Error> =
            gml_multi_curve_property.try_into();

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
        let multi_curve_result: Result<MultiCurveProperty, egml::io::Error> =
            gml_multi_curve_property.try_into();

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
        let multi_curve_result: Result<MultiCurveProperty, egml::io::Error> =
            gml_multi_curve_property.try_into();

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

    abstract_physical_space.set_point_cloud(point_cloud);

    Ok(abstract_physical_space)
}

pub fn serialize_abstract_physical_space(
    abstract_physical_space: &AbstractPhysicalSpace,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts =
        serialize_abstract_space(abstract_physical_space.abstract_space(), formatting)?;

    if let Some(raw) = serialize_inner(
        GmlAbstractPhysicalSpace::from(abstract_physical_space),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    xml_node_parts.content.extend(
        abstract_physical_space
            .point_cloud()
            .iter()
            .map(|x| serialize_point_cloud_property(x, formatting).map(XmlNodeContent::from))
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractPhysicalSpace {
    #[serde(
        rename = "lod1TerrainIntersectionCurve",
        skip_serializing_if = "Option::is_none"
    )]
    pub lod1_terrain_intersection_curve: Option<GmlMultiCurveProperty>,

    #[serde(
        rename = "lod2TerrainIntersectionCurve",
        skip_serializing_if = "Option::is_none"
    )]
    pub lod2_terrain_intersection_curve: Option<GmlMultiCurveProperty>,

    #[serde(
        rename = "lod3TerrainIntersectionCurve",
        skip_serializing_if = "Option::is_none"
    )]
    pub lod3_terrain_intersection_curve: Option<GmlMultiCurveProperty>,
}

impl From<&AbstractPhysicalSpace> for GmlAbstractPhysicalSpace {
    fn from(item: &AbstractPhysicalSpace) -> Self {
        Self {
            lod1_terrain_intersection_curve: item.lod1_terrain_intersection_curve().map(Into::into),
            lod2_terrain_intersection_curve: item.lod2_terrain_intersection_curve().map(Into::into),
            lod3_terrain_intersection_curve: item.lod3_terrain_intersection_curve().map(Into::into),
        }
    }
}
