use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_element::flatten_abstract_filling_element;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractFillingElementMut, Window};

pub fn flatten_window(mut window: Window, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_filling_element(window.abstract_filling_element_mut(), city_model_arena);

    city_model_arena.insert_feature(window.into()).into()
}
