use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_city_object::flatten_abstract_city_object;
use crate::model::core::{AbstractSpaceBoundary, AsAbstractCityObjectMut};

pub fn flatten_abstract_space_boundary(
    abstract_space_boundary: &mut AbstractSpaceBoundary,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_city_object(
        abstract_space_boundary.abstract_city_object_mut(),
        city_model_arena,
    )
}
