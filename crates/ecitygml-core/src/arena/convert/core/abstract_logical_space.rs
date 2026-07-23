use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_space::flatten_abstract_space;
use crate::model::core::{AbstractLogicalSpace, AsAbstractSpaceMut};

pub fn flatten_abstract_logical_space(
    abstract_logical_space: &mut AbstractLogicalSpace,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_space(
        abstract_logical_space.abstract_space_mut(),
        city_model_arena,
    )
}
