use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::clearance_space::flatten_clearance_space;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::ClearanceSpaceProperty;

pub fn flatten_clearance_space_property(
    clearance_space_property: &mut ClearanceSpaceProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = clearance_space_property.take_object() else {
        return;
    };

    let internal_key = flatten_clearance_space(object, city_model_arena);
    clearance_space_property.set_key(internal_key);
}
