use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_unoccupied_space;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractUnoccupiedSpaceMut;
use crate::model::transportation::AuxiliaryTrafficSpace;

pub fn flatten_auxiliary_traffic_space(
    mut auxiliary_traffic_space: AuxiliaryTrafficSpace,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_unoccupied_space(
        auxiliary_traffic_space.abstract_unoccupied_space_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(auxiliary_traffic_space.into())
        .into()
}
