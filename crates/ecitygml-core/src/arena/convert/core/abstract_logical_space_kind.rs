use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::flatten_abstract_building_subdivision_kind;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractLogicalSpaceKind;

pub fn flatten_abstract_logical_space_kind(
    abstract_logical_space_kind: AbstractLogicalSpaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_logical_space_kind {
        AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
            flatten_abstract_building_subdivision_kind(x, city_model_arena)
        }
    }
}
