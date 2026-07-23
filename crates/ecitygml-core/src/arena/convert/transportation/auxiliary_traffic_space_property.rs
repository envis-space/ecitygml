use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::auxiliary_traffic_space::flatten_auxiliary_traffic_space;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::AuxiliaryTrafficSpaceProperty;

pub fn flatten_auxiliary_traffic_space_property(
    auxiliary_traffic_space_property: &mut AuxiliaryTrafficSpaceProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = auxiliary_traffic_space_property.take_object() else {
        return;
    };

    let internal_key = flatten_auxiliary_traffic_space(object, city_model_arena);
    auxiliary_traffic_space_property.set_key(internal_key);
}
