use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_space_boundary;
use crate::model::core::AsAbstractSpaceBoundaryMut;
use crate::model::relief::AbstractReliefComponent;

pub fn flatten_abstract_relief_component(
    abstract_relief_component: &mut AbstractReliefComponent,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_space_boundary(
        abstract_relief_component.abstract_space_boundary_mut(),
        city_model_arena,
    )
}
