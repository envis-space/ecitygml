use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::flatten_building_room;
use crate::arena::convert::transportation::flatten_abstract_transportation_space_kind;
use crate::arena::convert::transportation::{
    flatten_auxiliary_traffic_space, flatten_clearance_space, flatten_hole, flatten_traffic_space,
};
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractUnoccupiedSpaceKind;

pub fn flatten_abstract_unoccupied_space_kind(
    abstract_unoccupied_space_kind: AbstractUnoccupiedSpaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_unoccupied_space_kind {
        AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => {
            flatten_auxiliary_traffic_space(x, city_model_arena)
        }
        AbstractUnoccupiedSpaceKind::BuildingRoom(x) => flatten_building_room(x, city_model_arena),
        AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => {
            flatten_clearance_space(x, city_model_arena)
        }
        AbstractUnoccupiedSpaceKind::Hole(x) => flatten_hole(x, city_model_arena),
        AbstractUnoccupiedSpaceKind::TrafficSpace(x) => flatten_traffic_space(x, city_model_arena),
        AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
            flatten_abstract_transportation_space_kind(x, city_model_arena)
        }
    }
}
