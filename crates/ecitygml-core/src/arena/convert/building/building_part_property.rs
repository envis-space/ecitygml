use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building_part::flatten_building_part;
use crate::model::building::BuildingPartProperty;
use crate::model::common::arena::HasArenaPropertiesMut;

pub fn flatten_building_part_property(
    building_part_property: &mut BuildingPartProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = building_part_property.take_object() else {
        return;
    };

    let internal_key = flatten_building_part(object, city_model_arena);
    building_part_property.set_key(internal_key);
}
