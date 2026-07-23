use std::collections::HashMap;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

pub fn get_namespace_pairs() -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::new();

    for ns in CitygmlNamespace::iter() {
        let key = match ns.default_prefix() {
            Some(prefix) => format!("xmlns:{prefix}"),
            None => "xmlns".to_string(),
        };
        pairs.push((key, ns.url().to_string()));
    }

    for ns in GeneralNamespace::iter() {
        if let Some(prefix) = ns.default_prefix() {
            pairs.push((format!("xmlns:{prefix}"), ns.url().to_string()));
        }
    }

    let schema_location = generate_schema_location_header_string();
    pairs.push((
        "xsi:schemaLocation".to_string(),
        schema_location.trim().to_string(),
    ));

    pairs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
pub enum CitygmlNamespace {
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

impl CitygmlNamespace {
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

    pub fn schema_location(&self) -> Option<&'static str> {
        match self {
            CitygmlNamespace::Appearance => {
                Some("http://www.opengis.net/citygml/appearance/3.0/appearance.xsd")
            }
            CitygmlNamespace::Bridge => {
                Some("http://www.opengis.net/citygml/bridge/3.0/bridge.xsd")
            }
            CitygmlNamespace::Building => {
                Some("http://www.opengis.net/citygml/building/3.0/building.xsd")
            }
            CitygmlNamespace::PointCloud => {
                Some("http://www.opengis.net/citygml/pointcloud/3.0/pointCloud.xsd")
            }
            CitygmlNamespace::CityFurniture => {
                Some("http://www.opengis.net/citygml/cityfurniture/3.0/cityFurniture.xsd")
            }
            CitygmlNamespace::CityObjectGroup => {
                Some("http://www.opengis.net/citygml/cityobjectgroup/3.0/cityObjectGroup.xsd")
            }
            CitygmlNamespace::Construction => {
                Some("http://www.opengis.net/citygml/construction/3.0/construction.xsd")
            }
            CitygmlNamespace::Core => None,
            CitygmlNamespace::Dynamizer => {
                Some("http://www.opengis.net/citygml/dynamizer/3.0/dynamizer.xsd")
            }
            CitygmlNamespace::Generics => {
                Some("http://www.opengis.net/citygml/generics/3.0/generics.xsd")
            }
            CitygmlNamespace::LandUse => {
                Some("http://www.opengis.net/citygml/landuse/3.0/landUse.xsd")
            }
            CitygmlNamespace::Relief => {
                Some("http://www.opengis.net/citygml/relief/3.0/relief.xsd")
            }
            CitygmlNamespace::Transportation => {
                Some("http://www.opengis.net/citygml/transportation/3.0/transportation.xsd")
            }
            CitygmlNamespace::Tunnel => {
                Some("http://www.opengis.net/citygml/tunnel/3.0/tunnel.xsd")
            }
            CitygmlNamespace::Vegetation => {
                Some("http://www.opengis.net/citygml/vegetation/3.0/vegetation.xsd")
            }
            CitygmlNamespace::Versioning => {
                Some("http://www.opengis.net/citygml/versioning/3.0/versioning.xsd")
            }
            CitygmlNamespace::WaterBody => {
                Some("http://www.opengis.net/citygml/waterbody/3.0/waterBody.xsd")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
pub enum GeneralNamespace {
    Gml,
    Xsi,
    XLink,
    Ct,
    XAl,
}

impl GeneralNamespace {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Gml => "http://www.opengis.net/gml/3.2",
            Self::Xsi => "http://www.w3.org/2001/XMLSchema-instance",
            Self::XLink => "http://www.w3.org/1999/xlink",
            Self::Ct => "http://schemas.opengis.net/citygml/xAL/3.0/CommonTypes.xsd",
            Self::XAl => "http://schemas.opengis.net/citygml/xAL/3.0/xAL.xsd",
        }
    }

    pub fn default_prefix(&self) -> Option<&'static str> {
        match self {
            Self::Gml => Some("gml"),
            Self::Xsi => Some("xsi"),
            Self::XLink => Some("xlink"),
            Self::Ct => Some("ct"),
            Self::XAl => Some("xAL"),
        }
    }
}

pub fn generate_schema_location_header_string() -> String {
    let mut schema_locations = String::new();
    for ns in CitygmlNamespace::iter() {
        if let Some(schema_location) = ns.schema_location() {
            let current_schema_location_pair = format!(" {} {}", ns.url(), schema_location);
            schema_locations.push_str(&current_schema_location_pair)
        }
    }

    schema_locations
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceContext {
    pub prefixes: HashMap<CitygmlNamespace, Option<String>>,
}

impl NamespaceContext {
    pub fn new() -> Self {
        let prefixes: HashMap<CitygmlNamespace, Option<String>> = CitygmlNamespace::iter()
            .map(|ns| (ns, ns.default_prefix().as_ref().map(|x| x.to_string())))
            .collect();

        Self { prefixes }
    }
}
