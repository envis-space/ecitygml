use crate::Error;
use crate::gml::codec::core::abstract_point_cloud_property::{
    deserialize_abstract_point_cloud_property, serialize_abstract_point_cloud_property,
};
use crate::gml::codec::core::{deserialize_abstract_space, serialize_abstract_space};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement, collect_gml_child_lenient};
use ecitygml_core::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut, AsAbstractSpace,
};
use egml::io::codec::geometry::aggregates::{
    deserialize_multi_curve_property, serialize_multi_curve_property,
};
use egml::io::util::collect_child;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_physical_space(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractPhysicalSpace, Error> {
    let mut abstract_space_result = None;
    let mut _parsed_result = None;
    let mut lod1_terrain_intersection_curve_result = None;
    let mut lod2_terrain_intersection_curve_result = None;
    let mut lod3_terrain_intersection_curve_result = None;
    let mut point_cloud_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_space_result = Some(deserialize_abstract_space(xml_document, spans));
        });
        s.spawn(|_| {
            _parsed_result = Some(
                de::from_reader::<_, GmlAbstractPhysicalSpace>(xml_document).map_err(Error::from),
            );
        });
        s.spawn(|_| {
            lod1_terrain_intersection_curve_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod1TerrainIntersectionCurveProperty.into(),
                deserialize_multi_curve_property,
            ));
        });
        s.spawn(|_| {
            lod2_terrain_intersection_curve_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod2TerrainIntersectionCurveProperty.into(),
                deserialize_multi_curve_property,
            ));
        });
        s.spawn(|_| {
            lod3_terrain_intersection_curve_result = Some(collect_gml_child_lenient(
                xml_document,
                spans,
                CityGmlElement::Lod3TerrainIntersectionCurveProperty.into(),
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

    let abstract_space =
        abstract_space_result.expect("rayon::scope guarantees all spawns complete")?;
    let _parsed = _parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let lod1_terrain_intersection_curve = lod1_terrain_intersection_curve_result
        .expect("rayon::scope guarantees all spawns complete");
    let lod2_terrain_intersection_curve = lod2_terrain_intersection_curve_result
        .expect("rayon::scope guarantees all spawns complete");
    let lod3_terrain_intersection_curve = lod3_terrain_intersection_curve_result
        .expect("rayon::scope guarantees all spawns complete");
    let point_cloud = point_cloud_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_physical_space = AbstractPhysicalSpace::from_abstract_space(abstract_space);
    abstract_physical_space.set_lod1_terrain_intersection_curve(lod1_terrain_intersection_curve);
    abstract_physical_space.set_lod2_terrain_intersection_curve(lod2_terrain_intersection_curve);
    abstract_physical_space.set_lod3_terrain_intersection_curve(lod3_terrain_intersection_curve);
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

    if let Some(prop) = abstract_physical_space.lod1_terrain_intersection_curve() {
        xml_node_parts.content.push(
            serialize_multi_curve_property(
                prop,
                formatting,
                CityGmlElement::Lod1TerrainIntersectionCurveProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_physical_space.lod2_terrain_intersection_curve() {
        xml_node_parts.content.push(
            serialize_multi_curve_property(
                prop,
                formatting,
                CityGmlElement::Lod2TerrainIntersectionCurveProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    if let Some(prop) = abstract_physical_space.lod3_terrain_intersection_curve() {
        xml_node_parts.content.push(
            serialize_multi_curve_property(
                prop,
                formatting,
                CityGmlElement::Lod3TerrainIntersectionCurveProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }
    xml_node_parts.content.extend(
        abstract_physical_space
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
pub struct GmlAbstractPhysicalSpace {}

impl From<&AbstractPhysicalSpace> for GmlAbstractPhysicalSpace {
    fn from(_item: &AbstractPhysicalSpace) -> Self {
        Self {}
    }
}
