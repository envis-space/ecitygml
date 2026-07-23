use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::section::flatten_section;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::transportation::SectionProperty;

pub fn flatten_section_property(
    section_property: &mut SectionProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = section_property.take_object() else {
        return;
    };

    let internal_key = flatten_section(object, city_model_arena);
    section_property.set_key(internal_key);
}
