use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface_kind;
use crate::arena::convert::relief::flatten_abstract_relief_component_kind;
use crate::arena::convert::relief::flatten_relief_feature;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractSpaceBoundaryKind;

pub fn flatten_abstract_space_boundary_kind(
    abstract_space_boundary_kind: AbstractSpaceBoundaryKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_space_boundary_kind {
        AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => {
            flatten_abstract_thematic_surface_kind(x, city_model_arena)
        }
        AbstractSpaceBoundaryKind::ReliefFeature(x) => flatten_relief_feature(x, city_model_arena),
        AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => {
            flatten_abstract_relief_component_kind(x, city_model_arena)
        }
    }
}
