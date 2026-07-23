use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_feature_with_lifespan::flatten_abstract_feature_with_lifespan;
use crate::model::core::{AbstractAppearance, AsAbstractFeatureWithLifespanMut};

pub fn flatten_abstract_appearance(
    abstract_appearance: &mut AbstractAppearance,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_feature_with_lifespan(
        abstract_appearance.abstract_feature_with_lifespan_mut(),
        city_model_arena,
    )
}
