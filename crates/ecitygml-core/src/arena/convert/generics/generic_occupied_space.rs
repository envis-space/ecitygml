use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractOccupiedSpaceMut;
use crate::model::generics::GenericOccupiedSpace;

pub fn flatten_generic_occupied_space(
    mut generic_occupied_space: GenericOccupiedSpace,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_occupied_space(
        generic_occupied_space.abstract_occupied_space_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(generic_occupied_space.into())
        .into()
}
