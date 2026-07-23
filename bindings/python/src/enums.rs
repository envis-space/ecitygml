use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

// ---------------------------------------------------------------------------
// LevelOfDetail
// ---------------------------------------------------------------------------

#[gen_stub_pyclass_enum]
#[pyclass(name = "LevelOfDetail", eq, frozen, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyLevelOfDetail {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[pymethods]
impl PyLevelOfDetail {
    #[getter]
    pub fn value(&self) -> i32 {
        *self as i32
    }

    pub fn __repr__(&self) -> &'static str {
        match self {
            PyLevelOfDetail::Zero => "LevelOfDetail.Zero",
            PyLevelOfDetail::One => "LevelOfDetail.One",
            PyLevelOfDetail::Two => "LevelOfDetail.Two",
            PyLevelOfDetail::Three => "LevelOfDetail.Three",
        }
    }
}

impl From<ecitygml_rs::model::common::LevelOfDetail> for PyLevelOfDetail {
    fn from(lod: ecitygml_rs::model::common::LevelOfDetail) -> Self {
        match lod {
            ecitygml_rs::model::common::LevelOfDetail::Zero => PyLevelOfDetail::Zero,
            ecitygml_rs::model::common::LevelOfDetail::One => PyLevelOfDetail::One,
            ecitygml_rs::model::common::LevelOfDetail::Two => PyLevelOfDetail::Two,
            ecitygml_rs::model::common::LevelOfDetail::Three => PyLevelOfDetail::Three,
        }
    }
}

// ---------------------------------------------------------------------------
// FeatureType
// ---------------------------------------------------------------------------

#[gen_stub_pyclass_enum]
#[pyclass(name = "FeatureType", eq, frozen, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyFeatureType {
    Appearance,
    AuxiliaryTrafficArea,
    AuxiliaryTrafficSpace,
    Bridge,
    BridgeConstructiveElement,
    BridgeFurniture,
    BridgeInstallation,
    BridgePart,
    BridgeRoom,
    Building,
    BuildingConstructiveElement,
    BuildingFurniture,
    BuildingInstallation,
    BuildingPart,
    BuildingRoom,
    BuildingUnit,
    CeilingSurface,
    CityFurniture,
    CityModel,
    CityObjectGroup,
    ClearanceSpace,
    ClosureSurface,
    Door,
    DoorSurface,
    FloorSurface,
    GenericLogicalSpace,
    GenericOccupiedSpace,
    GenericThematicSurface,
    GenericUnoccupiedSpace,
    GroundSurface,
    Hole,
    HoleSurface,
    HollowSpace,
    InteriorWallSurface,
    Intersection,
    LandUse,
    Marking,
    OtherConstruction,
    OuterCeilingSurface,
    OuterFloorSurface,
    PlantCover,
    PointCloud,
    Railway,
    ReliefFeature,
    Road,
    RoofSurface,
    Section,
    SolitaryVegetationObject,
    Square,
    Storey,
    TinRelief,
    Track,
    TrafficArea,
    TrafficSpace,
    Tunnel,
    TunnelConstructiveElement,
    TunnelFurniture,
    TunnelInstallation,
    TunnelPart,
    GeoreferencedTexture,
    ParameterizedTexture,
    WallSurface,
    WaterBody,
    WaterGroundSurface,
    WaterSurface,
    Waterway,
    Window,
    WindowSurface,
    X3DMaterial,
}

#[pymethods]
impl PyFeatureType {
    pub fn __repr__(&self) -> String {
        format!("FeatureType.{:?}", self)
    }
}

impl std::fmt::Debug for PyFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Appearance => "Appearance",
            Self::AuxiliaryTrafficArea => "AuxiliaryTrafficArea",
            Self::AuxiliaryTrafficSpace => "AuxiliaryTrafficSpace",
            Self::Bridge => "Bridge",
            Self::BridgeConstructiveElement => "BridgeConstructiveElement",
            Self::BridgeFurniture => "BridgeFurniture",
            Self::BridgeInstallation => "BridgeInstallation",
            Self::BridgePart => "BridgePart",
            Self::BridgeRoom => "BridgeRoom",
            Self::Building => "Building",
            Self::BuildingConstructiveElement => "BuildingConstructiveElement",
            Self::BuildingFurniture => "BuildingFurniture",
            Self::BuildingInstallation => "BuildingInstallation",
            Self::BuildingPart => "BuildingPart",
            Self::BuildingRoom => "BuildingRoom",
            Self::BuildingUnit => "BuildingUnit",
            Self::CeilingSurface => "CeilingSurface",
            Self::CityFurniture => "CityFurniture",
            Self::CityModel => "CityModel",
            Self::CityObjectGroup => "CityObjectGroup",
            Self::ClearanceSpace => "ClearanceSpace",
            Self::ClosureSurface => "ClosureSurface",
            Self::Door => "Door",
            Self::DoorSurface => "DoorSurface",
            Self::FloorSurface => "FloorSurface",
            Self::GenericLogicalSpace => "GenericLogicalSpace",
            Self::GenericOccupiedSpace => "GenericOccupiedSpace",
            Self::GenericThematicSurface => "GenericThematicSurface",
            Self::GenericUnoccupiedSpace => "GenericUnoccupiedSpace",
            Self::GroundSurface => "GroundSurface",
            Self::Hole => "Hole",
            Self::HoleSurface => "HoleSurface",
            Self::HollowSpace => "HollowSpace",
            Self::InteriorWallSurface => "InteriorWallSurface",
            Self::Intersection => "Intersection",
            Self::LandUse => "LandUse",
            Self::Marking => "Marking",
            Self::OtherConstruction => "OtherConstruction",
            Self::OuterCeilingSurface => "OuterCeilingSurface",
            Self::OuterFloorSurface => "OuterFloorSurface",
            Self::PlantCover => "PlantCover",
            Self::PointCloud => "PointCloud",
            Self::Railway => "Railway",
            Self::ReliefFeature => "ReliefFeature",
            Self::Road => "Road",
            Self::RoofSurface => "RoofSurface",
            Self::Section => "Section",
            Self::SolitaryVegetationObject => "SolitaryVegetationObject",
            Self::Square => "Square",
            Self::Storey => "Storey",
            Self::TinRelief => "TinRelief",
            Self::Track => "Track",
            Self::TrafficArea => "TrafficArea",
            Self::TrafficSpace => "TrafficSpace",
            Self::Tunnel => "Tunnel",
            Self::TunnelConstructiveElement => "TunnelConstructiveElement",
            Self::TunnelFurniture => "TunnelFurniture",
            Self::TunnelInstallation => "TunnelInstallation",
            Self::TunnelPart => "TunnelPart",
            Self::GeoreferencedTexture => "GeoreferencedTexture",
            Self::ParameterizedTexture => "ParameterizedTexture",
            Self::WallSurface => "WallSurface",
            Self::WaterBody => "WaterBody",
            Self::WaterGroundSurface => "WaterGroundSurface",
            Self::WaterSurface => "WaterSurface",
            Self::Waterway => "Waterway",
            Self::Window => "Window",
            Self::WindowSurface => "WindowSurface",
            Self::X3DMaterial => "X3DMaterial",
        };
        write!(f, "{}", s)
    }
}

impl From<ecitygml_rs::model::common::FeatureType> for PyFeatureType {
    fn from(ft: ecitygml_rs::model::common::FeatureType) -> Self {
        use ecitygml_rs::model::common::FeatureType as F;
        match ft {
            F::Appearance => Self::Appearance,
            F::AuxiliaryTrafficArea => Self::AuxiliaryTrafficArea,
            F::AuxiliaryTrafficSpace => Self::AuxiliaryTrafficSpace,
            F::Bridge => Self::Bridge,
            F::BridgeConstructiveElement => Self::BridgeConstructiveElement,
            F::BridgeFurniture => Self::BridgeFurniture,
            F::BridgeInstallation => Self::BridgeInstallation,
            F::BridgePart => Self::BridgePart,
            F::BridgeRoom => Self::BridgeRoom,
            F::Building => Self::Building,
            F::BuildingConstructiveElement => Self::BuildingConstructiveElement,
            F::BuildingFurniture => Self::BuildingFurniture,
            F::BuildingInstallation => Self::BuildingInstallation,
            F::BuildingPart => Self::BuildingPart,
            F::BuildingRoom => Self::BuildingRoom,
            F::BuildingUnit => Self::BuildingUnit,
            F::CeilingSurface => Self::CeilingSurface,
            F::CityFurniture => Self::CityFurniture,
            F::CityModel => Self::CityModel,
            F::CityObjectGroup => Self::CityObjectGroup,
            F::ClearanceSpace => Self::ClearanceSpace,
            F::ClosureSurface => Self::ClosureSurface,
            F::Door => Self::Door,
            F::DoorSurface => Self::DoorSurface,
            F::FloorSurface => Self::FloorSurface,
            F::GenericLogicalSpace => Self::GenericLogicalSpace,
            F::GenericOccupiedSpace => Self::GenericOccupiedSpace,
            F::GenericThematicSurface => Self::GenericThematicSurface,
            F::GenericUnoccupiedSpace => Self::GenericUnoccupiedSpace,
            F::GroundSurface => Self::GroundSurface,
            F::Hole => Self::Hole,
            F::HoleSurface => Self::HoleSurface,
            F::HollowSpace => Self::HollowSpace,
            F::InteriorWallSurface => Self::InteriorWallSurface,
            F::Intersection => Self::Intersection,
            F::LandUse => Self::LandUse,
            F::Marking => Self::Marking,
            F::OtherConstruction => Self::OtherConstruction,
            F::OuterCeilingSurface => Self::OuterCeilingSurface,
            F::OuterFloorSurface => Self::OuterFloorSurface,
            F::PlantCover => Self::PlantCover,
            F::PointCloud => Self::PointCloud,
            F::Railway => Self::Railway,
            F::ReliefFeature => Self::ReliefFeature,
            F::Road => Self::Road,
            F::RoofSurface => Self::RoofSurface,
            F::Section => Self::Section,
            F::SolitaryVegetationObject => Self::SolitaryVegetationObject,
            F::Square => Self::Square,
            F::Storey => Self::Storey,
            F::TinRelief => Self::TinRelief,
            F::Track => Self::Track,
            F::TrafficArea => Self::TrafficArea,
            F::TrafficSpace => Self::TrafficSpace,
            F::Tunnel => Self::Tunnel,
            F::TunnelConstructiveElement => Self::TunnelConstructiveElement,
            F::TunnelFurniture => Self::TunnelFurniture,
            F::TunnelInstallation => Self::TunnelInstallation,
            F::TunnelPart => Self::TunnelPart,
            F::GeoreferencedTexture => Self::GeoreferencedTexture,
            F::ParameterizedTexture => Self::ParameterizedTexture,
            F::WallSurface => Self::WallSurface,
            F::WaterBody => Self::WaterBody,
            F::WaterGroundSurface => Self::WaterGroundSurface,
            F::WaterSurface => Self::WaterSurface,
            F::Waterway => Self::Waterway,
            F::Window => Self::Window,
            F::WindowSurface => Self::WindowSurface,
            F::X3DMaterial => Self::X3DMaterial,
        }
    }
}
