use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::flatten_appearance;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractAppearanceKind;

pub fn flatten_abstract_appearance_kind(
    abstract_appearance_kind: AbstractAppearanceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_appearance_kind {
        AbstractAppearanceKind::Appearance(x) => flatten_appearance(x, city_model_arena),
    }
}
