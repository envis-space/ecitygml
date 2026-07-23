use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::hole::flatten_hole;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::HoleProperty;

pub fn flatten_hole_property(
    hole_property: &mut HoleProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = hole_property.take_object() else {
        return;
    };

    let internal_key = flatten_hole(object, city_model_arena);
    hole_property.set_key(internal_key);
}
