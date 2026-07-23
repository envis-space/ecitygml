use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_space_boundary_kind::flatten_abstract_space_boundary_kind;
use crate::arena::convert::core::abstract_space_kind::flatten_abstract_space_kind;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractCityObjectKind;

pub fn flatten_abstract_city_object_kind(
    abstract_city_object_kind: AbstractCityObjectKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_city_object_kind {
        AbstractCityObjectKind::AbstractSpaceKind(x) => {
            flatten_abstract_space_kind(x, city_model_arena)
        }
        AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => {
            flatten_abstract_space_boundary_kind(x, city_model_arena)
        }
    }
}
