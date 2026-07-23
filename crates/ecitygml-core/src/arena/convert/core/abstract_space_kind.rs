use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_logical_space_kind::flatten_abstract_logical_space_kind;
use crate::arena::convert::core::abstract_physical_space_kind::flatten_abstract_physical_space_kind;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractSpaceKind;

pub fn flatten_abstract_space_kind(
    abstract_space_kind: AbstractSpaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_space_kind {
        AbstractSpaceKind::AbstractLogicalSpaceKind(x) => {
            flatten_abstract_logical_space_kind(x, city_model_arena)
        }
        AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => {
            flatten_abstract_physical_space_kind(x, city_model_arena)
        }
    }
}
