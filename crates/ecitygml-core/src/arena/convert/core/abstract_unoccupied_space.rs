use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_physical_space::flatten_abstract_physical_space;
use crate::model::core::{AbstractUnoccupiedSpace, AsAbstractPhysicalSpaceMut};

pub fn flatten_abstract_unoccupied_space(
    abstract_unoccupied_space: &mut AbstractUnoccupiedSpace,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_physical_space(
        abstract_unoccupied_space.abstract_physical_space_mut(),
        city_model_arena,
    )
}
