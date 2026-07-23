use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_feature::flatten_abstract_feature;
use crate::model::core::AbstractFeatureWithLifespan;

pub fn flatten_abstract_feature_with_lifespan(
    abstract_feature_with_lifespan: &mut AbstractFeatureWithLifespan,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_feature(
        &mut abstract_feature_with_lifespan.abstract_feature,
        city_model_arena,
    );
}
