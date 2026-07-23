use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_surface_data_kind::flatten_abstract_surface_data_kind;
use crate::model::appearance::AbstractSurfaceDataProperty;
use crate::model::common::arena::HasArenaPropertiesMut;

pub fn flatten_abstract_surface_data_property(
    abstract_surface_data_property: &mut AbstractSurfaceDataProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_surface_data_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_surface_data_kind(object, city_model_arena);
    abstract_surface_data_property.set_key(internal_key);
}
