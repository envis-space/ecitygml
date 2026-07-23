use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building_constructive_element::flatten_building_constructive_element;
use crate::model::building::BuildingConstructiveElementProperty;
use crate::model::common::arena::HasArenaPropertiesMut;

pub fn flatten_building_constructive_element_property(
    building_constructive_element_property: &mut BuildingConstructiveElementProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = building_constructive_element_property.take_object() else {
        return;
    };

    let internal_key = flatten_building_constructive_element(object, city_model_arena);
    building_constructive_element_property.set_key(internal_key);
}
