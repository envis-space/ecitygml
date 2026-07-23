use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::marking::flatten_marking;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::MarkingProperty;

pub fn flatten_marking_property(
    marking_property: &mut MarkingProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = marking_property.take_object() else {
        return;
    };

    let internal_key = flatten_marking(object, city_model_arena);
    marking_property.set_key(internal_key);
}
