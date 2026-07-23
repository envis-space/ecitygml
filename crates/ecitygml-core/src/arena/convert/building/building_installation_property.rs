use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building_installation::flatten_building_installation;
use crate::model::building::BuildingInstallationProperty;
use crate::model::common::arena::HasArenaPropertiesMut;

pub fn flatten_building_installation_property(
    building_installation_property: &mut BuildingInstallationProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = building_installation_property.take_object() else {
        return;
    };

    let internal_key = flatten_building_installation(object, city_model_arena);
    building_installation_property.set_key(internal_key);
}
