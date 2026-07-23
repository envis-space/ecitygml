use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::abstract_building::flatten_abstract_building;
use crate::arena::convert::building::building_part_property::flatten_building_part_property;
use crate::model::building::AsAbstractBuildingMut;
use crate::model::building::Building;
use crate::model::common::arena::InternalKey;

pub fn flatten_building(
    mut building: Building,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_building(building.abstract_building_mut(), city_model_arena);

    for prop in building.building_parts_mut() {
        flatten_building_part_property(prop, city_model_arena);
    }

    city_model_arena.insert_feature(building.into()).into()
}
