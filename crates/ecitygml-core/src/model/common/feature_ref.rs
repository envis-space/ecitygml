use crate::model::building::{
    Building, BuildingConstructiveElement, BuildingInstallation, BuildingRoom, BuildingUnit, Storey,
};
use crate::model::city_furniture::CityFurniture;
use crate::model::common::FeatureType;
use crate::model::construction::{
    CeilingSurface, Door, DoorSurface, FloorSurface, GroundSurface, InteriorWallSurface,
    OuterCeilingSurface, OuterFloorSurface, RoofSurface, WallSurface, Window, WindowSurface,
};
use crate::model::core::{
    AbstractCityObject, AbstractFeature, AbstractFeatureWithLifespan, AsAbstractCityObject,
    AsAbstractFeature, AsAbstractFeatureWithLifespan, CityObjectKind, ClosureSurface,
};
use crate::model::relief::{ReliefFeature, TinRelief};
use crate::model::transportation::{
    AuxiliaryTrafficArea, AuxiliaryTrafficSpace, Intersection, Marking, Road, Section, TrafficArea,
    TrafficSpace,
};
use crate::model::vegetation::SolitaryVegetationObject;

#[derive(Debug, Clone, Copy)]
pub enum FeatureRef<'a> {
    AuxiliaryTrafficArea(&'a AuxiliaryTrafficArea),
    AuxiliaryTrafficSpace(&'a AuxiliaryTrafficSpace),
    Building(&'a Building),
    BuildingConstructiveElement(&'a BuildingConstructiveElement),
    BuildingInstallation(&'a BuildingInstallation),
    BuildingRoom(&'a BuildingRoom),
    BuildingUnit(&'a BuildingUnit),
    CeilingSurface(&'a CeilingSurface),
    CityFurniture(&'a CityFurniture),
    ClosureSurface(&'a ClosureSurface),
    Door(&'a Door),
    DoorSurface(&'a DoorSurface),
    FloorSurface(&'a FloorSurface),
    GroundSurface(&'a GroundSurface),
    InteriorWallSurface(&'a InteriorWallSurface),
    Intersection(&'a Intersection),
    Marking(&'a Marking),
    OuterCeilingSurface(&'a OuterCeilingSurface),
    OuterFloorSurface(&'a OuterFloorSurface),
    ReliefFeature(&'a ReliefFeature),
    Road(&'a Road),
    RoofSurface(&'a RoofSurface),
    Section(&'a Section),
    SolitaryVegetationObject(&'a SolitaryVegetationObject),
    Storey(&'a Storey),
    TinRelief(&'a TinRelief),
    TrafficArea(&'a TrafficArea),
    TrafficSpace(&'a TrafficSpace),
    WallSurface(&'a WallSurface),
    Window(&'a Window),
    WindowSurface(&'a WindowSurface),
}

impl<'a> AsAbstractCityObject for FeatureRef<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            FeatureRef::AuxiliaryTrafficArea(x) => x.abstract_city_object(),
            FeatureRef::AuxiliaryTrafficSpace(x) => x.abstract_city_object(),
            FeatureRef::Building(x) => x.abstract_city_object(),
            FeatureRef::BuildingConstructiveElement(x) => x.abstract_city_object(),
            FeatureRef::BuildingInstallation(x) => x.abstract_city_object(),
            FeatureRef::BuildingRoom(x) => x.abstract_city_object(),
            FeatureRef::BuildingUnit(x) => x.abstract_city_object(),
            FeatureRef::CeilingSurface(x) => x.abstract_city_object(),
            FeatureRef::CityFurniture(x) => x.abstract_city_object(),
            FeatureRef::ClosureSurface(x) => x.abstract_city_object(),
            FeatureRef::Door(x) => x.abstract_city_object(),
            FeatureRef::DoorSurface(x) => x.abstract_city_object(),
            FeatureRef::FloorSurface(x) => x.abstract_city_object(),
            FeatureRef::GroundSurface(x) => x.abstract_city_object(),
            FeatureRef::InteriorWallSurface(x) => x.abstract_city_object(),
            FeatureRef::Intersection(x) => x.abstract_city_object(),
            FeatureRef::Marking(x) => x.abstract_city_object(),
            FeatureRef::OuterCeilingSurface(x) => x.abstract_city_object(),
            FeatureRef::OuterFloorSurface(x) => x.abstract_city_object(),
            FeatureRef::ReliefFeature(x) => x.abstract_city_object(),
            FeatureRef::Road(x) => x.abstract_city_object(),
            FeatureRef::RoofSurface(x) => x.abstract_city_object(),
            FeatureRef::Section(x) => x.abstract_city_object(),
            FeatureRef::SolitaryVegetationObject(x) => x.abstract_city_object(),
            FeatureRef::Storey(x) => x.abstract_city_object(),
            FeatureRef::TinRelief(x) => x.abstract_city_object(),
            FeatureRef::TrafficArea(x) => x.abstract_city_object(),
            FeatureRef::TrafficSpace(x) => x.abstract_city_object(),
            FeatureRef::WallSurface(x) => x.abstract_city_object(),
            FeatureRef::Window(x) => x.abstract_city_object(),
            FeatureRef::WindowSurface(x) => x.abstract_city_object(),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for FeatureRef<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            FeatureRef::AuxiliaryTrafficArea(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::AuxiliaryTrafficSpace(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Building(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::BuildingConstructiveElement(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::BuildingInstallation(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::BuildingRoom(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::BuildingUnit(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::CeilingSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::CityFurniture(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::ClosureSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Door(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::DoorSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::FloorSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::GroundSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::InteriorWallSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Intersection(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Marking(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::OuterCeilingSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::OuterFloorSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::ReliefFeature(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Road(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::RoofSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Section(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::SolitaryVegetationObject(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Storey(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::TinRelief(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::TrafficArea(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::TrafficSpace(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::WallSurface(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::Window(x) => x.abstract_feature_with_lifespan(),
            FeatureRef::WindowSurface(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl<'a> AsAbstractFeature for FeatureRef<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            FeatureRef::AuxiliaryTrafficArea(x) => x.abstract_feature(),
            FeatureRef::AuxiliaryTrafficSpace(x) => x.abstract_feature(),
            FeatureRef::Building(x) => x.abstract_feature(),
            FeatureRef::BuildingConstructiveElement(x) => x.abstract_feature(),
            FeatureRef::BuildingInstallation(x) => x.abstract_feature(),
            FeatureRef::BuildingRoom(x) => x.abstract_feature(),
            FeatureRef::BuildingUnit(x) => x.abstract_feature(),
            FeatureRef::CeilingSurface(x) => x.abstract_feature(),
            FeatureRef::CityFurniture(x) => x.abstract_feature(),
            FeatureRef::ClosureSurface(x) => x.abstract_feature(),
            FeatureRef::Door(x) => x.abstract_feature(),
            FeatureRef::DoorSurface(x) => x.abstract_feature(),
            FeatureRef::FloorSurface(x) => x.abstract_feature(),
            FeatureRef::GroundSurface(x) => x.abstract_feature(),
            FeatureRef::InteriorWallSurface(x) => x.abstract_feature(),
            FeatureRef::Intersection(x) => x.abstract_feature(),
            FeatureRef::Marking(x) => x.abstract_feature(),
            FeatureRef::OuterCeilingSurface(x) => x.abstract_feature(),
            FeatureRef::OuterFloorSurface(x) => x.abstract_feature(),
            FeatureRef::ReliefFeature(x) => x.abstract_feature(),
            FeatureRef::Road(x) => x.abstract_feature(),
            FeatureRef::RoofSurface(x) => x.abstract_feature(),
            FeatureRef::Section(x) => x.abstract_feature(),
            FeatureRef::SolitaryVegetationObject(x) => x.abstract_feature(),
            FeatureRef::Storey(x) => x.abstract_feature(),
            FeatureRef::TinRelief(x) => x.abstract_feature(),
            FeatureRef::TrafficArea(x) => x.abstract_feature(),
            FeatureRef::TrafficSpace(x) => x.abstract_feature(),
            FeatureRef::WallSurface(x) => x.abstract_feature(),
            FeatureRef::Window(x) => x.abstract_feature(),
            FeatureRef::WindowSurface(x) => x.abstract_feature(),
        }
    }
}

impl<'a> FeatureRef<'a> {
    pub fn to_city_object_kind(self) -> CityObjectKind {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.clone().into(),
            Self::AuxiliaryTrafficSpace(x) => x.clone().into(),
            Self::Building(x) => x.clone().into(),
            Self::BuildingConstructiveElement(x) => x.clone().into(),
            Self::BuildingInstallation(x) => x.clone().into(),
            Self::BuildingRoom(x) => x.clone().into(),
            Self::BuildingUnit(x) => x.clone().into(),
            Self::CeilingSurface(x) => x.clone().into(),
            Self::CityFurniture(x) => x.clone().into(),
            Self::ClosureSurface(x) => x.clone().into(),
            Self::Door(x) => x.clone().into(),
            Self::DoorSurface(x) => x.clone().into(),
            Self::FloorSurface(x) => x.clone().into(),
            Self::GroundSurface(x) => x.clone().into(),
            Self::InteriorWallSurface(x) => x.clone().into(),
            Self::Intersection(x) => x.clone().into(),
            Self::Marking(x) => x.clone().into(),
            Self::OuterCeilingSurface(x) => x.clone().into(),
            Self::OuterFloorSurface(x) => x.clone().into(),
            Self::ReliefFeature(x) => x.clone().into(),
            Self::Road(x) => x.clone().into(),
            Self::RoofSurface(x) => x.clone().into(),
            Self::Section(x) => x.clone().into(),
            Self::SolitaryVegetationObject(x) => x.clone().into(),
            Self::Storey(x) => x.clone().into(),
            Self::TinRelief(x) => x.clone().into(),
            Self::TrafficArea(x) => x.clone().into(),
            Self::TrafficSpace(x) => x.clone().into(),
            Self::WallSurface(x) => x.clone().into(),
            Self::Window(x) => x.clone().into(),
            Self::WindowSurface(x) => x.clone().into(),
        }
    }

    pub fn feature_type(&self) -> FeatureType {
        match self {
            FeatureRef::AuxiliaryTrafficArea(_) => FeatureType::AuxiliaryTrafficArea,
            FeatureRef::AuxiliaryTrafficSpace(_) => FeatureType::AuxiliaryTrafficSpace,
            FeatureRef::Building(_) => FeatureType::Building,
            FeatureRef::BuildingConstructiveElement(_) => FeatureType::BuildingConstructiveElement,
            FeatureRef::BuildingInstallation(_) => FeatureType::BuildingInstallation,
            FeatureRef::BuildingRoom(_) => FeatureType::BuildingRoom,
            FeatureRef::BuildingUnit(_) => FeatureType::BuildingUnit,
            FeatureRef::CeilingSurface(_) => FeatureType::CeilingSurface,
            FeatureRef::CityFurniture(_) => FeatureType::CityFurniture,
            FeatureRef::ClosureSurface(_) => FeatureType::ClosureSurface,
            FeatureRef::Door(_) => FeatureType::Door,
            FeatureRef::DoorSurface(_) => FeatureType::DoorSurface,
            FeatureRef::FloorSurface(_) => FeatureType::FloorSurface,
            FeatureRef::GroundSurface(_) => FeatureType::GroundSurface,
            FeatureRef::InteriorWallSurface(_) => FeatureType::InteriorWallSurface,
            FeatureRef::Intersection(_) => FeatureType::Intersection,
            FeatureRef::Marking(_) => FeatureType::Marking,
            FeatureRef::OuterCeilingSurface(_) => FeatureType::OuterCeilingSurface,
            FeatureRef::OuterFloorSurface(_) => FeatureType::OuterFloorSurface,
            FeatureRef::ReliefFeature(_) => FeatureType::ReliefFeature,
            FeatureRef::Road(_) => FeatureType::Road,
            FeatureRef::RoofSurface(_) => FeatureType::RoofSurface,
            FeatureRef::Section(_) => FeatureType::Section,
            FeatureRef::SolitaryVegetationObject(_) => FeatureType::SolitaryVegetationObject,
            FeatureRef::Storey(_) => FeatureType::Storey,
            FeatureRef::TinRelief(_) => FeatureType::TinRelief,
            FeatureRef::TrafficArea(_) => FeatureType::TrafficArea,
            FeatureRef::TrafficSpace(_) => FeatureType::TrafficSpace,
            FeatureRef::WallSurface(_) => FeatureType::WallSurface,
            FeatureRef::Window(_) => FeatureType::Window,
            FeatureRef::WindowSurface(_) => FeatureType::WindowSurface,
        }
    }
}
