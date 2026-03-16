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
// CityObjectClass
// ---------------------------------------------------------------------------

#[gen_stub_pyclass_enum]
#[pyclass(name = "CityObjectClass", eq, frozen, skip_from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyCityObjectClass {
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
    CityObjectGroup,
    ClearanceSpace,
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
    Marking,
    OtherConstruction,
    OuterCeilingSurface,
    OuterFloorSurface,
    PlantCover,
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
    WallSurface,
    WaterBody,
    WaterGroundSurface,
    WaterSurface,
    Waterway,
    Window,
    WindowSurface,
}

#[pymethods]
impl PyCityObjectClass {
    pub fn __repr__(&self) -> String {
        format!("CityObjectClass.{:?}", self)
    }
}

impl std::fmt::Debug for PyCityObjectClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
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
            Self::CityObjectGroup => "CityObjectGroup",
            Self::ClearanceSpace => "ClearanceSpace",
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
            Self::Marking => "Marking",
            Self::OtherConstruction => "OtherConstruction",
            Self::OuterCeilingSurface => "OuterCeilingSurface",
            Self::OuterFloorSurface => "OuterFloorSurface",
            Self::PlantCover => "PlantCover",
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
            Self::WallSurface => "WallSurface",
            Self::WaterBody => "WaterBody",
            Self::WaterGroundSurface => "WaterGroundSurface",
            Self::WaterSurface => "WaterSurface",
            Self::Waterway => "Waterway",
            Self::Window => "Window",
            Self::WindowSurface => "WindowSurface",
        };
        write!(f, "{}", s)
    }
}

impl From<ecitygml_rs::model::common::CityObjectClass> for PyCityObjectClass {
    fn from(c: ecitygml_rs::model::common::CityObjectClass) -> Self {
        use ecitygml_rs::model::common::CityObjectClass as C;
        match c {
            C::AuxiliaryTrafficArea => Self::AuxiliaryTrafficArea,
            C::AuxiliaryTrafficSpace => Self::AuxiliaryTrafficSpace,
            C::Bridge => Self::Bridge,
            C::BridgeConstructiveElement => Self::BridgeConstructiveElement,
            C::BridgeFurniture => Self::BridgeFurniture,
            C::BridgeInstallation => Self::BridgeInstallation,
            C::BridgePart => Self::BridgePart,
            C::BridgeRoom => Self::BridgeRoom,
            C::Building => Self::Building,
            C::BuildingConstructiveElement => Self::BuildingConstructiveElement,
            C::BuildingFurniture => Self::BuildingFurniture,
            C::BuildingInstallation => Self::BuildingInstallation,
            C::BuildingPart => Self::BuildingPart,
            C::BuildingRoom => Self::BuildingRoom,
            C::BuildingUnit => Self::BuildingUnit,
            C::CeilingSurface => Self::CeilingSurface,
            C::CityFurniture => Self::CityFurniture,
            C::CityObjectGroup => Self::CityObjectGroup,
            C::ClearanceSpace => Self::ClearanceSpace,
            C::Door => Self::Door,
            C::DoorSurface => Self::DoorSurface,
            C::FloorSurface => Self::FloorSurface,
            C::GenericLogicalSpace => Self::GenericLogicalSpace,
            C::GenericOccupiedSpace => Self::GenericOccupiedSpace,
            C::GenericThematicSurface => Self::GenericThematicSurface,
            C::GenericUnoccupiedSpace => Self::GenericUnoccupiedSpace,
            C::GroundSurface => Self::GroundSurface,
            C::Hole => Self::Hole,
            C::HoleSurface => Self::HoleSurface,
            C::HollowSpace => Self::HollowSpace,
            C::InteriorWallSurface => Self::InteriorWallSurface,
            C::Intersection => Self::Intersection,
            C::Marking => Self::Marking,
            C::OtherConstruction => Self::OtherConstruction,
            C::OuterCeilingSurface => Self::OuterCeilingSurface,
            C::OuterFloorSurface => Self::OuterFloorSurface,
            C::PlantCover => Self::PlantCover,
            C::Railway => Self::Railway,
            C::ReliefFeature => Self::ReliefFeature,
            C::Road => Self::Road,
            C::RoofSurface => Self::RoofSurface,
            C::Section => Self::Section,
            C::SolitaryVegetationObject => Self::SolitaryVegetationObject,
            C::Square => Self::Square,
            C::Storey => Self::Storey,
            C::TinRelief => Self::TinRelief,
            C::Track => Self::Track,
            C::TrafficArea => Self::TrafficArea,
            C::TrafficSpace => Self::TrafficSpace,
            C::Tunnel => Self::Tunnel,
            C::TunnelConstructiveElement => Self::TunnelConstructiveElement,
            C::TunnelFurniture => Self::TunnelFurniture,
            C::TunnelInstallation => Self::TunnelInstallation,
            C::TunnelPart => Self::TunnelPart,
            C::WallSurface => Self::WallSurface,
            C::WaterBody => Self::WaterBody,
            C::WaterGroundSurface => Self::WaterGroundSurface,
            C::WaterSurface => Self::WaterSurface,
            C::Waterway => Self::Waterway,
            C::Window => Self::Window,
            C::WindowSurface => Self::WindowSurface,
        }
    }
}
