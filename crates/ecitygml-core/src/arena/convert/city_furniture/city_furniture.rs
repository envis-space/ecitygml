use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractOccupiedSpaceMut;

pub fn flatten_city_furniture(
    mut city_furniture: CityFurniture,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_occupied_space(
        city_furniture.abstract_occupied_space_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(city_furniture.into())
        .into()
}
