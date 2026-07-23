use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::traffic_space::flatten_traffic_space;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::TrafficSpaceProperty;

pub fn flatten_traffic_space_property(
    traffic_space_property: &mut TrafficSpaceProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = traffic_space_property.take_object() else {
        return;
    };

    let internal_key = flatten_traffic_space(object, city_model_arena);
    traffic_space_property.set_key(internal_key);
}
