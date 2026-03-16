use crate::Error;
use crate::gml::parser::building::{
    deserialize_building, deserialize_building_constructive_element,
    deserialize_building_installation, deserialize_building_room, deserialize_storey,
};
use crate::gml::parser::city_furniture::deserialize_city_furniture;
use crate::gml::parser::construction::{
    deserialize_door_surface, deserialize_ground_surface, deserialize_roof_surface,
    deserialize_wall_surface, deserialize_window_surface,
};
use crate::gml::parser::relief::deserialize_relief_feature;
use crate::gml::parser::relief::deserialize_tin_relief;
use crate::gml::parser::transportation::deserialize_auxiliary_traffic_space;
use crate::gml::parser::transportation::{
    deserialize_auxiliary_traffic_area, deserialize_traffic_space,
};
use crate::gml::parser::transportation::{deserialize_intersection, deserialize_section};
use crate::gml::parser::transportation::{deserialize_road, deserialize_traffic_area};
use crate::gml::parser::vegetation::deserialize_solitary_vegetation_object;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use quick_xml::reader::Span;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::ops::Range;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CityObjectSpan {
    pub(crate) city_object_class: CityObjectClass,
    pub(crate) span: Range<usize>,
}

impl CityObjectSpan {
    pub fn new(city_object_class: CityObjectClass, span: Span) -> Self {
        Self {
            city_object_class,
            span: span.start as usize..span.end as usize,
        }
    }
}

pub(crate) fn deserialize_city_object(
    xml_document: &[u8],
    city_object_span: CityObjectSpan,
) -> Result<CityObjectKind, Error> {
    let xml_document_slice = &xml_document[city_object_span.span];

    let city_object: CityObjectKind = match city_object_span.city_object_class {
        CityObjectClass::AuxiliaryTrafficArea => {
            let auxiliary_traffic_area = deserialize_auxiliary_traffic_area(xml_document_slice)?;
            CityObjectKind::AuxiliaryTrafficArea(auxiliary_traffic_area)
        }
        CityObjectClass::AuxiliaryTrafficSpace => {
            let auxiliary_traffic_space = deserialize_auxiliary_traffic_space(xml_document_slice)?;
            CityObjectKind::AuxiliaryTrafficSpace(auxiliary_traffic_space)
        }
        CityObjectClass::Building => {
            let building = deserialize_building(xml_document_slice)?;
            CityObjectKind::Building(building)
        }
        CityObjectClass::BuildingConstructiveElement => {
            let building_constructive_element =
                deserialize_building_constructive_element(xml_document_slice)?;
            CityObjectKind::BuildingConstructiveElement(building_constructive_element)
        }
        CityObjectClass::BuildingInstallation => {
            let building_installation = deserialize_building_installation(xml_document_slice)?;
            CityObjectKind::BuildingInstallation(building_installation)
        }
        CityObjectClass::BuildingRoom => {
            let building_room = deserialize_building_room(xml_document_slice)?;
            CityObjectKind::BuildingRoom(building_room)
        }
        CityObjectClass::CityFurniture => {
            let city_furniture = deserialize_city_furniture(xml_document_slice)?;
            CityObjectKind::CityFurniture(city_furniture)
        }
        CityObjectClass::Road => {
            let road = deserialize_road(xml_document_slice)?;
            CityObjectKind::Road(road)
        }
        CityObjectClass::Section => {
            let section = deserialize_section(xml_document_slice)?;
            CityObjectKind::Section(section)
        }
        CityObjectClass::SolitaryVegetationObject => {
            let solitary_vegetation_object =
                deserialize_solitary_vegetation_object(xml_document_slice)?;
            CityObjectKind::SolitaryVegetationObject(solitary_vegetation_object)
        }
        CityObjectClass::Storey => {
            let storey = deserialize_storey(xml_document_slice)?;
            CityObjectKind::Storey(storey)
        }
        CityObjectClass::Intersection => {
            let intersection = deserialize_intersection(xml_document_slice)?;
            CityObjectKind::Intersection(intersection)
        }
        CityObjectClass::TrafficSpace => {
            let traffic_space = deserialize_traffic_space(xml_document_slice)?;
            CityObjectKind::TrafficSpace(traffic_space)
        }
        CityObjectClass::TrafficArea => {
            let traffic_area = deserialize_traffic_area(xml_document_slice)?;
            CityObjectKind::TrafficArea(traffic_area)
        }
        CityObjectClass::GroundSurface => {
            let ground_surface = deserialize_ground_surface(xml_document_slice)?;
            CityObjectKind::GroundSurface(ground_surface)
        }
        CityObjectClass::ReliefFeature => {
            let relief_feature = deserialize_relief_feature(xml_document_slice)?;
            CityObjectKind::ReliefFeature(relief_feature)
        }
        CityObjectClass::TinRelief => {
            let tin_relief = deserialize_tin_relief(xml_document_slice)?;
            CityObjectKind::TinRelief(tin_relief)
        }
        CityObjectClass::RoofSurface => {
            let roof_surface = deserialize_roof_surface(xml_document_slice)?;
            CityObjectKind::RoofSurface(roof_surface)
        }
        CityObjectClass::WallSurface => {
            let wall_surface = deserialize_wall_surface(xml_document_slice)?;
            CityObjectKind::WallSurface(wall_surface)
        }
        CityObjectClass::WindowSurface => {
            let window_surface = deserialize_window_surface(xml_document_slice)?;
            CityObjectKind::WindowSurface(window_surface)
        }
        CityObjectClass::DoorSurface => {
            let door_surface = deserialize_door_surface(xml_document_slice)?;
            CityObjectKind::DoorSurface(door_surface)
        }
        _ => {
            return Err(Error::UnknownElementNode(format!(
                "{:?}",
                city_object_span.city_object_class
            )));
        }
    };

    Ok(city_object)
}

pub fn city_object_class_from_bytes(local_name: &[u8]) -> Result<CityObjectClass, Error> {
    match local_name {
        b"AuxiliaryTrafficArea" => Ok(CityObjectClass::AuxiliaryTrafficArea),
        b"AuxiliaryTrafficSpace" => Ok(CityObjectClass::AuxiliaryTrafficSpace),
        b"Building" => Ok(CityObjectClass::Building),
        b"BuildingConstructiveElement" => Ok(CityObjectClass::BuildingConstructiveElement),
        b"BuildingInstallation" => Ok(CityObjectClass::BuildingInstallation),
        b"BuildingRoom" => Ok(CityObjectClass::BuildingRoom),
        b"CityFurniture" => Ok(CityObjectClass::CityFurniture),
        b"DoorSurface" => Ok(CityObjectClass::DoorSurface),
        b"GroundSurface" => Ok(CityObjectClass::GroundSurface),
        b"Intersection" => Ok(CityObjectClass::Intersection),
        b"ReliefFeature" => Ok(CityObjectClass::ReliefFeature),
        b"Road" => Ok(CityObjectClass::Road),
        b"RoofSurface" => Ok(CityObjectClass::RoofSurface),
        b"Section" => Ok(CityObjectClass::Section),
        b"SolitaryVegetationObject" => Ok(CityObjectClass::SolitaryVegetationObject),
        b"Storey" => Ok(CityObjectClass::Storey),
        b"TINRelief" => Ok(CityObjectClass::TinRelief),
        b"TrafficArea" => Ok(CityObjectClass::TrafficArea),
        b"TrafficSpace" => Ok(CityObjectClass::TrafficSpace),
        b"WallSurface" => Ok(CityObjectClass::WallSurface),
        b"WindowSurface" => Ok(CityObjectClass::WindowSurface),
        _ => Err(Error::UnknownElementNode(
            String::from_utf8_lossy(local_name).into_owned(),
        )),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
pub enum Namespace {
    Appearance,
    Bridge,
    Building,
    PointCloud,
    CityFurniture,
    CityObjectGroup,
    Construction,
    Core,
    Dynamizer,
    Generics,
    LandUse,
    Relief,
    Transportation,
    Tunnel,
    Vegetation,
    Versioning,
    WaterBody,
}

impl Namespace {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Appearance => "http://www.opengis.net/citygml/appearance/3.0",
            Self::Bridge => "http://www.opengis.net/citygml/bridge/3.0",
            Self::Building => "http://www.opengis.net/citygml/building/3.0",
            Self::PointCloud => "http://www.opengis.net/citygml/pointcloud/3.0",
            Self::CityFurniture => "http://www.opengis.net/citygml/cityfurniture/3.0",
            Self::CityObjectGroup => "http://www.opengis.net/citygml/cityobjectgroup/3.0",
            Self::Construction => "http://www.opengis.net/citygml/construction/3.0",
            Self::Core => "http://www.opengis.net/citygml/3.0",
            Self::Dynamizer => "http://www.opengis.net/citygml/dynamizer/3.0",
            Self::Generics => "http://www.opengis.net/citygml/generics/3.0",
            Self::LandUse => "http://www.opengis.net/citygml/landuse/3.0",
            Self::Relief => "http://www.opengis.net/citygml/relief/3.0",
            Self::Transportation => "http://www.opengis.net/citygml/transportation/3.0",
            Self::Tunnel => "http://www.opengis.net/citygml/tunnel/3.0",
            Self::Vegetation => "http://www.opengis.net/citygml/vegetation/3.0",
            Self::Versioning => "http://www.opengis.net/citygml/versioning/3.0",
            Self::WaterBody => "http://www.opengis.net/citygml/waterbody/3.0",
        }
    }

    pub fn default_prefix(&self) -> Option<&'static str> {
        match self {
            Self::Appearance => Some("app"),
            Self::Bridge => Some("brid"),
            Self::Building => Some("bldg"),
            Self::PointCloud => Some("pcl"),
            Self::CityFurniture => Some("frn"),
            Self::CityObjectGroup => Some("grp"),
            Self::Construction => Some("con"),
            Self::Core => None,
            Self::Dynamizer => Some("dyn"),
            Self::Generics => Some("gen"),
            Self::LandUse => Some("luse"),
            Self::Relief => Some("dem"),
            Self::Transportation => Some("tran"),
            Self::Tunnel => Some("tun"),
            Self::Vegetation => Some("veg"),
            Self::Versioning => Some("vers"),
            Self::WaterBody => Some("wtr"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceContext {
    pub prefixes: HashMap<Namespace, Option<String>>,
}

impl NamespaceContext {
    pub fn new() -> Self {
        let prefixes: HashMap<Namespace, Option<String>> = Namespace::iter()
            .map(|ns| (ns, ns.default_prefix().as_ref().map(|x| x.to_string())))
            .collect();

        Self { prefixes }
    }
}

pub fn deserialize_space_separated_f64<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect())
}
