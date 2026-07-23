use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_appearance_kind::flatten_abstract_appearance_kind;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::core::AbstractAppearanceProperty;

pub fn flatten_abstract_appearance_property(
    abstract_appearance_property: &mut AbstractAppearanceProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_appearance_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_appearance_kind(object, city_model_arena);
    abstract_appearance_property.set_key(internal_key);
}
