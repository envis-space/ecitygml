use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_element_kind::flatten_abstract_filling_element_kind;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::construction::AbstractFillingElementProperty;

pub fn flatten_abstract_filling_element_property(
    abstract_filling_element_property: &mut AbstractFillingElementProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_filling_element_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_filling_element_kind(object, city_model_arena);
    abstract_filling_element_property.set_key(internal_key);
}
