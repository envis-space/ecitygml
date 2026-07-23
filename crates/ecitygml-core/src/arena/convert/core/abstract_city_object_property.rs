use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_city_object_kind::flatten_abstract_city_object_kind;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::core::AbstractCityObjectProperty;

pub fn flatten_abstract_city_object_property(
    abstract_city_object_property: &mut AbstractCityObjectProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(city_object) = abstract_city_object_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_city_object_kind(city_object, city_model_arena);
    abstract_city_object_property.set_key(internal_key);
}
