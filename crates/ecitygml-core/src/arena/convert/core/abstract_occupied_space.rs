use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_physical_space::flatten_abstract_physical_space;
use crate::model::core::{AbstractOccupiedSpace, AsAbstractPhysicalSpaceMut};

pub fn flatten_abstract_occupied_space(
    abstract_occupied_space: &mut AbstractOccupiedSpace,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_physical_space(
        abstract_occupied_space.abstract_physical_space_mut(),
        city_model_arena,
    )
}
