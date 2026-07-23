use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::intersection::flatten_intersection;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::IntersectionProperty;

pub fn flatten_intersection_property(
    intersection_property: &mut IntersectionProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = intersection_property.take_object() else {
        return;
    };

    let internal_key = flatten_intersection(object, city_model_arena);
    intersection_property.set_key(internal_key);
}
