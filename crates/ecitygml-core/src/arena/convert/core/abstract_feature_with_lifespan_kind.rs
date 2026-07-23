use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_appearance_kind::flatten_abstract_appearance_kind;
use crate::arena::convert::core::abstract_city_object_kind::flatten_abstract_city_object_kind;
use crate::arena::convert::core::flatten_city_model;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractFeatureWithLifespanKind;

pub fn flatten_abstract_feature_with_lifespan_kind(
    abstract_feature_with_lifespan_kind: AbstractFeatureWithLifespanKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_feature_with_lifespan_kind {
        AbstractFeatureWithLifespanKind::AbstractAppearanceKind(x) => {
            flatten_abstract_appearance_kind(x, city_model_arena)
        }
        AbstractFeatureWithLifespanKind::CityModel(x) => flatten_city_model(x, city_model_arena),
        AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x) => {
            flatten_abstract_city_object_kind(x, city_model_arena)
        }
    }
}
