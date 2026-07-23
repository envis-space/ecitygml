use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_element_property::flatten_abstract_filling_element_property;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::construction::{AbstractConstructiveElement, AsAbstractConstructiveElementMut};
use crate::model::core::AsAbstractOccupiedSpaceMut;

pub fn flatten_abstract_constructive_element(
    abstract_constructive_element: &mut AbstractConstructiveElement,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_occupied_space(
        abstract_constructive_element.abstract_occupied_space_mut(),
        city_model_arena,
    );

    for prop in abstract_constructive_element.fillings_mut() {
        flatten_abstract_filling_element_property(prop, city_model_arena);
    }
}
