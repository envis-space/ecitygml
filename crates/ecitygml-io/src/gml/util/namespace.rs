use std::collections::HashMap;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

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
