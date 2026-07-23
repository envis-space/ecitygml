use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_space_boundary_kind::flatten_abstract_space_boundary_kind;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::core::AbstractSpaceBoundaryProperty;

pub fn flatten_abstract_space_boundary_property(
    abstract_space_boundary_property: &mut AbstractSpaceBoundaryProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_space_boundary_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_space_boundary_kind(object, city_model_arena);
    abstract_space_boundary_property.set_key(internal_key);
}
