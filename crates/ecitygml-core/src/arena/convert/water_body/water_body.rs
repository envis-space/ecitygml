use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractOccupiedSpaceMut;
use crate::model::water_body::WaterBody;

pub fn flatten_water_body(
    mut water_body: WaterBody,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_occupied_space(water_body.abstract_occupied_space_mut(), city_model_arena);

    city_model_arena.insert_feature(water_body.into()).into()
}
