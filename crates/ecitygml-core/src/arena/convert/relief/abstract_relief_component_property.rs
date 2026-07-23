use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::relief::abstract_relief_component_kind::flatten_abstract_relief_component_kind;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::relief::AbstractReliefComponentProperty;

pub fn flatten_abstract_relief_component_property(
    abstract_relief_component_property: &mut AbstractReliefComponentProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_relief_component_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_relief_component_kind(object, city_model_arena);
    abstract_relief_component_property.set_key(internal_key);
}
