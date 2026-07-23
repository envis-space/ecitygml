use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::core::AsAbstractOccupiedSpaceMut;
use crate::model::vegetation::AbstractVegetationObject;

pub fn flatten_abstract_vegetation_object(
    abstract_vegetation_object: &mut AbstractVegetationObject,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_occupied_space(
        abstract_vegetation_object.abstract_occupied_space_mut(),
        city_model_arena,
    )
}
