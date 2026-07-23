use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::construction::AbstractFillingElement;
use crate::model::core::AsAbstractOccupiedSpaceMut;

pub fn flatten_abstract_filling_element(
    abstract_filling_element: &mut AbstractFillingElement,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_occupied_space(
        abstract_filling_element.abstract_occupied_space_mut(),
        city_model_arena,
    )
}
