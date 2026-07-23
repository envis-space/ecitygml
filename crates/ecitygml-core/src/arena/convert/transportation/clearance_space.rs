use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_unoccupied_space;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractUnoccupiedSpaceMut;
use crate::model::transportation::ClearanceSpace;

pub fn flatten_clearance_space(
    mut clearance_space: ClearanceSpace,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_unoccupied_space(
        clearance_space.abstract_unoccupied_space_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(clearance_space.into())
        .into()
}
