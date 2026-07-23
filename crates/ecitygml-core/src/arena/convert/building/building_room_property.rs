use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building_room::flatten_building_room;
use crate::model::building::BuildingRoomProperty;
use crate::model::common::arena::HasArenaPropertiesMut;

pub fn flatten_building_room_property(
    building_room_property: &mut BuildingRoomProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = building_room_property.take_object() else {
        return;
    };

    let internal_key = flatten_building_room(object, city_model_arena);
    building_room_property.set_key(internal_key);
}
