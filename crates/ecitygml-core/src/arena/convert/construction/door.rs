use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_element::flatten_abstract_filling_element;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractFillingElementMut, Door};

pub fn flatten_door(mut door: Door, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_filling_element(door.abstract_filling_element_mut(), city_model_arena);

    city_model_arena.insert_feature(door.into()).into()
}
