use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::abstract_building::flatten_abstract_building;
use crate::model::building::AsAbstractBuildingMut;
use crate::model::building::BuildingPart;
use crate::model::common::arena::InternalKey;

pub fn flatten_building_part(
    mut building_part: BuildingPart,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_building(building_part.abstract_building_mut(), city_model_arena);

    city_model_arena.insert_feature(building_part.into()).into()
}
