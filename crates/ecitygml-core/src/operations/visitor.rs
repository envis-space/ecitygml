use crate::model::building::{
    Building, BuildingConstructiveElement, BuildingInstallation, BuildingRoom, Storey,
};
use crate::model::city_furniture::CityFurniture;
use crate::model::construction::{
    DoorSurface, GroundSurface, RoofSurface, WallSurface, WindowSurface,
};
use crate::model::core::{AsAbstractFeature, CityModel};
use crate::model::relief::{ReliefFeature, TinRelief};
use crate::model::transportation::{
    AuxiliaryTrafficArea, AuxiliaryTrafficSpace, Intersection, Road, Section, TrafficArea,
    TrafficSpace,
};
use crate::model::vegetation::SolitaryVegetationObject;

pub trait Visitable: AsAbstractFeature {
    fn accept<V: Visitor>(&self, visitor: &mut V);
}

pub trait Visitor {
    type Output: Default;

    fn visit_city_model(&mut self, _v: &CityModel) -> Self::Output {
        Self::Output::default()
    }

    fn visit_city_furniture(&mut self, _v: &CityFurniture) -> Self::Output {
        Self::Output::default()
    }

    fn visit_building(&mut self, _v: &Building) -> Self::Output {
        Self::Output::default()
    }

    fn visit_building_constructive_element(
        &mut self,
        _v: &BuildingConstructiveElement,
    ) -> Self::Output {
        Self::Output::default()
    }

    fn visit_building_installation(&mut self, _v: &BuildingInstallation) -> Self::Output {
        Self::Output::default()
    }

    fn visit_building_room(&mut self, _v: &BuildingRoom) -> Self::Output {
        Self::Output::default()
    }

    fn visit_roof_surface(&mut self, _v: &RoofSurface) -> Self::Output {
        Self::Output::default()
    }

    fn visit_ground_surface(&mut self, _v: &GroundSurface) -> Self::Output {
        Self::Output::default()
    }

    fn visit_wall_surface(&mut self, _v: &WallSurface) -> Self::Output {
        Self::Output::default()
    }

    fn visit_window_surface(&mut self, _v: &WindowSurface) -> Self::Output {
        Self::Output::default()
    }

    fn visit_door_surface(&mut self, _v: &DoorSurface) -> Self::Output {
        Self::Output::default()
    }

    fn visit_solitary_vegetation_object(&mut self, _v: &SolitaryVegetationObject) -> Self::Output {
        Self::Output::default()
    }

    fn visit_storey(&mut self, _v: &Storey) -> Self::Output {
        Self::Output::default()
    }

    fn visit_relief_feature(&mut self, _v: &ReliefFeature) -> Self::Output {
        Self::Output::default()
    }

    fn visit_road(&mut self, _v: &Road) -> Self::Output {
        Self::Output::default()
    }

    fn visit_section(&mut self, _v: &Section) -> Self::Output {
        Self::Output::default()
    }

    fn visit_intersection(&mut self, _v: &Intersection) -> Self::Output {
        Self::Output::default()
    }

    fn visit_tin_relief(&mut self, _v: &TinRelief) -> Self::Output {
        Self::Output::default()
    }

    fn visit_traffic_space(&mut self, _v: &TrafficSpace) -> Self::Output {
        Self::Output::default()
    }

    fn visit_auxiliary_traffic_space(&mut self, _v: &AuxiliaryTrafficSpace) -> Self::Output {
        Self::Output::default()
    }

    fn visit_traffic_area(&mut self, _v: &TrafficArea) -> Self::Output {
        Self::Output::default()
    }

    fn visit_auxiliary_traffic_area(&mut self, _v: &AuxiliaryTrafficArea) -> Self::Output {
        Self::Output::default()
    }
}
