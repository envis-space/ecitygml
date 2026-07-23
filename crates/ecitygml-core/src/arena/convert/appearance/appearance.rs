use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_surface_data_property::flatten_abstract_surface_data_property;
use crate::arena::convert::core::flatten_abstract_appearance;
use crate::model::appearance::Appearance;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractAppearanceMut;

pub fn flatten_appearance(
    mut appearance: Appearance,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_appearance(appearance.abstract_appearance_mut(), city_model_arena);

    for prop in appearance.surface_data_mut() {
        flatten_abstract_surface_data_property(prop, city_model_arena);
    }

    city_model_arena.insert_feature(appearance.into()).into()
}
