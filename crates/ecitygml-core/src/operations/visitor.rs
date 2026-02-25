use crate::model::building::{Building, BuildingConstructiveElement};
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

pub struct Interpreter;

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
}
impl Visitor for Interpreter {
    type Output = ();

    fn visit_city_model(&mut self, _v: &CityModel) -> Self::Output {
        println!("hello city_model");
    }

    fn visit_city_furniture(&mut self, v: &CityFurniture) -> Self::Output {
        println!("hello city_furniture {}", v.id(),);
    }

    fn visit_building(&mut self, v: &Building) -> Self::Output {
        println!("hello building {}", v.id(),);
    }

    fn visit_building_constructive_element(
        &mut self,
        v: &BuildingConstructiveElement,
    ) -> Self::Output {
        println!("hello building_constructive_element {}", v.id(),);
    }

    fn visit_roof_surface(&mut self, v: &RoofSurface) -> Self::Output {
        println!("hello roof_surface {}", v.id());
    }

    fn visit_ground_surface(&mut self, v: &GroundSurface) -> Self::Output {
        println!("hello ground_surface {}", v.id());
    }

    fn visit_wall_surface(&mut self, v: &WallSurface) -> Self::Output {
        println!("hello wall_surface {}", v.id());
    }

    fn visit_window_surface(&mut self, v: &WindowSurface) -> Self::Output {
        println!("hello window_surface {}", v.id());
    }

    fn visit_door_surface(&mut self, v: &DoorSurface) -> Self::Output {
        println!("hello door_surface {}", v.id());
    }

    fn visit_solitary_vegetation_object(&mut self, v: &SolitaryVegetationObject) -> Self::Output {
        println!("hello solitary_vegetation_object {}", v.id());
    }

    fn visit_road(&mut self, v: &Road) -> Self::Output {
        println!("hello road {}", v.id());
    }

    fn visit_section(&mut self, v: &Section) -> Self::Output {
        println!("hello section {}", v.id());
    }

    fn visit_intersection(&mut self, v: &Intersection) -> Self::Output {
        println!("hello intersection {}", v.id());
    }

    fn visit_traffic_space(&mut self, v: &TrafficSpace) -> Self::Output {
        println!("hello traffic_space {}", v.id());
    }

    fn visit_auxiliary_traffic_space(&mut self, v: &AuxiliaryTrafficSpace) -> Self::Output {
        println!("hello auxiliary_traffic_space {}", v.id());
    }

    fn visit_traffic_area(&mut self, v: &TrafficArea) -> Self::Output {
        println!("hello traffic_area {}", v.id());
    }

    fn visit_auxiliary_traffic_area(&mut self, v: &AuxiliaryTrafficArea) -> Self::Output {
        println!("hello auxiliary_traffic_area {}", v.id());
    }
}
