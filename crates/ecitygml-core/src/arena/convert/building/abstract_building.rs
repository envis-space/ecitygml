use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::abstract_building_subdivision_property::flatten_abstract_building_subdivision_property;
use crate::arena::convert::building::building_constructive_element_property::flatten_building_constructive_element_property;
use crate::arena::convert::building::building_installation_property::flatten_building_installation_property;
use crate::arena::convert::building::building_room_property::flatten_building_room_property;
use crate::arena::convert::construction::flatten_abstract_construction;
use crate::model::building::{AbstractBuilding, AsAbstractBuildingMut};
use crate::model::construction::AsAbstractConstructionMut;

pub fn flatten_abstract_building(
    abstract_building: &mut AbstractBuilding,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_construction(
        abstract_building.abstract_construction_mut(),
        city_model_arena,
    );

    for prop in abstract_building.building_constructive_elements_mut() {
        flatten_building_constructive_element_property(prop, city_model_arena);
    }
    for prop in abstract_building.building_installations_mut() {
        flatten_building_installation_property(prop, city_model_arena);
    }
    for prop in abstract_building.building_rooms_mut() {
        flatten_building_room_property(prop, city_model_arena);
    }
    for prop in abstract_building.building_subdivisions_mut() {
        flatten_abstract_building_subdivision_property(prop, city_model_arena);
    }
}
