use crate::model::building::{
    Building, BuildingConstructiveElement, BuildingInstallation, BuildingRoom, BuildingUnit, Storey,
};
use crate::model::city_furniture::CityFurniture;
use crate::model::construction::{
    CeilingSurface, Door, DoorSurface, FloorSurface, GroundSurface, InteriorWallSurface,
    OuterCeilingSurface, OuterFloorSurface, RoofSurface, WallSurface, Window, WindowSurface,
};
use crate::model::core::{
    AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut, ClosureSurface,
};
use crate::model::relief::{ReliefFeature, TinRelief};
use crate::model::transportation::{
    AuxiliaryTrafficArea, AuxiliaryTrafficSpace, Intersection, Marking, Road, Section, TrafficArea,
    TrafficSpace,
};
use crate::model::vegetation::SolitaryVegetationObject;

#[derive(Debug)]
pub enum FeatureRefMut<'a> {
    AuxiliaryTrafficArea(&'a mut AuxiliaryTrafficArea),
    AuxiliaryTrafficSpace(&'a mut AuxiliaryTrafficSpace),
    Building(&'a mut Building),
    BuildingConstructiveElement(&'a mut BuildingConstructiveElement),
    BuildingInstallation(&'a mut BuildingInstallation),
    BuildingRoom(&'a mut BuildingRoom),
    BuildingUnit(&'a mut BuildingUnit),
    CeilingSurface(&'a mut CeilingSurface),
    CityFurniture(&'a mut CityFurniture),
    ClosureSurface(&'a mut ClosureSurface),
    Door(&'a mut Door),
    DoorSurface(&'a mut DoorSurface),
    FloorSurface(&'a mut FloorSurface),
    GroundSurface(&'a mut GroundSurface),
    InteriorWallSurface(&'a mut InteriorWallSurface),
    Intersection(&'a mut Intersection),
    Marking(&'a mut Marking),
    OuterCeilingSurface(&'a mut OuterCeilingSurface),
    OuterFloorSurface(&'a mut OuterFloorSurface),
    ReliefFeature(&'a mut ReliefFeature),
    Road(&'a mut Road),
    RoofSurface(&'a mut RoofSurface),
    Section(&'a mut Section),
    SolitaryVegetationObject(&'a mut SolitaryVegetationObject),
    Storey(&'a mut Storey),
    TinRelief(&'a mut TinRelief),
    TrafficArea(&'a mut TrafficArea),
    TrafficSpace(&'a mut TrafficSpace),
    WallSurface(&'a mut WallSurface),
    Window(&'a mut Window),
    WindowSurface(&'a mut WindowSurface),
}

impl<'a> AsAbstractFeature for FeatureRefMut<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            FeatureRefMut::AuxiliaryTrafficArea(x) => x.abstract_feature(),
            FeatureRefMut::AuxiliaryTrafficSpace(x) => x.abstract_feature(),
            FeatureRefMut::Building(x) => x.abstract_feature(),
            FeatureRefMut::BuildingConstructiveElement(x) => x.abstract_feature(),
            FeatureRefMut::BuildingInstallation(x) => x.abstract_feature(),
            FeatureRefMut::BuildingRoom(x) => x.abstract_feature(),
            FeatureRefMut::BuildingUnit(x) => x.abstract_feature(),
            FeatureRefMut::CeilingSurface(x) => x.abstract_feature(),
            FeatureRefMut::CityFurniture(x) => x.abstract_feature(),
            FeatureRefMut::ClosureSurface(x) => x.abstract_feature(),
            FeatureRefMut::Door(x) => x.abstract_feature(),
            FeatureRefMut::DoorSurface(x) => x.abstract_feature(),
            FeatureRefMut::FloorSurface(x) => x.abstract_feature(),
            FeatureRefMut::GroundSurface(x) => x.abstract_feature(),
            FeatureRefMut::InteriorWallSurface(x) => x.abstract_feature(),
            FeatureRefMut::Intersection(x) => x.abstract_feature(),
            FeatureRefMut::Marking(x) => x.abstract_feature(),
            FeatureRefMut::OuterCeilingSurface(x) => x.abstract_feature(),
            FeatureRefMut::OuterFloorSurface(x) => x.abstract_feature(),
            FeatureRefMut::ReliefFeature(x) => x.abstract_feature(),
            FeatureRefMut::Road(x) => x.abstract_feature(),
            FeatureRefMut::RoofSurface(x) => x.abstract_feature(),
            FeatureRefMut::Section(x) => x.abstract_feature(),
            FeatureRefMut::SolitaryVegetationObject(x) => x.abstract_feature(),
            FeatureRefMut::Storey(x) => x.abstract_feature(),
            FeatureRefMut::TinRelief(x) => x.abstract_feature(),
            FeatureRefMut::TrafficArea(x) => x.abstract_feature(),
            FeatureRefMut::TrafficSpace(x) => x.abstract_feature(),
            FeatureRefMut::WallSurface(x) => x.abstract_feature(),
            FeatureRefMut::Window(x) => x.abstract_feature(),
            FeatureRefMut::WindowSurface(x) => x.abstract_feature(),
        }
    }
}

impl<'a> AsAbstractFeatureMut for FeatureRefMut<'a> {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            FeatureRefMut::AuxiliaryTrafficArea(x) => x.abstract_feature_mut(),
            FeatureRefMut::AuxiliaryTrafficSpace(x) => x.abstract_feature_mut(),
            FeatureRefMut::Building(x) => x.abstract_feature_mut(),
            FeatureRefMut::BuildingConstructiveElement(x) => x.abstract_feature_mut(),
            FeatureRefMut::BuildingInstallation(x) => x.abstract_feature_mut(),
            FeatureRefMut::BuildingRoom(x) => x.abstract_feature_mut(),
            FeatureRefMut::BuildingUnit(x) => x.abstract_feature_mut(),
            FeatureRefMut::CeilingSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::CityFurniture(x) => x.abstract_feature_mut(),
            FeatureRefMut::ClosureSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::Door(x) => x.abstract_feature_mut(),
            FeatureRefMut::DoorSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::FloorSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::GroundSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::InteriorWallSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::Intersection(x) => x.abstract_feature_mut(),
            FeatureRefMut::Marking(x) => x.abstract_feature_mut(),
            FeatureRefMut::OuterCeilingSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::OuterFloorSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::ReliefFeature(x) => x.abstract_feature_mut(),
            FeatureRefMut::Road(x) => x.abstract_feature_mut(),
            FeatureRefMut::RoofSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::Section(x) => x.abstract_feature_mut(),
            FeatureRefMut::SolitaryVegetationObject(x) => x.abstract_feature_mut(),
            FeatureRefMut::Storey(x) => x.abstract_feature_mut(),
            FeatureRefMut::TinRelief(x) => x.abstract_feature_mut(),
            FeatureRefMut::TrafficArea(x) => x.abstract_feature_mut(),
            FeatureRefMut::TrafficSpace(x) => x.abstract_feature_mut(),
            FeatureRefMut::WallSurface(x) => x.abstract_feature_mut(),
            FeatureRefMut::Window(x) => x.abstract_feature_mut(),
            FeatureRefMut::WindowSurface(x) => x.abstract_feature_mut(),
        }
    }
}

impl<'a> FeatureRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            FeatureRefMut::AuxiliaryTrafficArea(x) => x.recompute_bounding_shape(),
            FeatureRefMut::AuxiliaryTrafficSpace(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Building(x) => x.recompute_bounding_shape(),
            FeatureRefMut::BuildingConstructiveElement(x) => x.recompute_bounding_shape(),
            FeatureRefMut::BuildingInstallation(x) => x.recompute_bounding_shape(),
            FeatureRefMut::BuildingRoom(x) => x.recompute_bounding_shape(),
            FeatureRefMut::BuildingUnit(x) => x.recompute_bounding_shape(),
            FeatureRefMut::CeilingSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::CityFurniture(x) => x.recompute_bounding_shape(),
            FeatureRefMut::ClosureSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Door(x) => x.recompute_bounding_shape(),
            FeatureRefMut::DoorSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::FloorSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::GroundSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::InteriorWallSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Intersection(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Marking(x) => x.recompute_bounding_shape(),
            FeatureRefMut::OuterCeilingSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::OuterFloorSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::ReliefFeature(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Road(x) => x.recompute_bounding_shape(),
            FeatureRefMut::RoofSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Section(x) => x.recompute_bounding_shape(),
            FeatureRefMut::SolitaryVegetationObject(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Storey(x) => x.recompute_bounding_shape(),
            FeatureRefMut::TinRelief(x) => x.recompute_bounding_shape(),
            FeatureRefMut::TrafficArea(x) => x.recompute_bounding_shape(),
            FeatureRefMut::TrafficSpace(x) => x.recompute_bounding_shape(),
            FeatureRefMut::WallSurface(x) => x.recompute_bounding_shape(),
            FeatureRefMut::Window(x) => x.recompute_bounding_shape(),
            FeatureRefMut::WindowSurface(x) => x.recompute_bounding_shape(),
        }
    }
}
