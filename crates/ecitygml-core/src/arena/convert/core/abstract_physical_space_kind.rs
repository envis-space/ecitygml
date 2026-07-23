use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_occupied_space_kind::flatten_abstract_occupied_space_kind;
use crate::arena::convert::core::abstract_unoccupied_space_kind::flatten_abstract_unoccupied_space_kind;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractPhysicalSpaceKind;

pub fn flatten_abstract_physical_space_kind(
    abstract_physical_space_kind: AbstractPhysicalSpaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_physical_space_kind {
        AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => {
            flatten_abstract_occupied_space_kind(x, city_model_arena)
        }
        AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => {
            flatten_abstract_unoccupied_space_kind(x, city_model_arena)
        }
    }
}
