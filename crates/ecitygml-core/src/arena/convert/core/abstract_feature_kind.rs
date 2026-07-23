use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::flatten_abstract_surface_data_kind;
use crate::arena::convert::core::abstract_feature_with_lifespan_kind::flatten_abstract_feature_with_lifespan_kind;
use crate::arena::convert::core::abstract_point_cloud_kind::flatten_abstract_point_cloud_kind;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractFeatureKind;

pub fn flatten_abstract_feature_kind(
    abstract_feature_kind: AbstractFeatureKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_feature_kind {
        AbstractFeatureKind::AbstractFeatureWithLifespanKind(x) => {
            flatten_abstract_feature_with_lifespan_kind(x, city_model_arena)
        }
        AbstractFeatureKind::AbstractSurfaceDataKind(x) => {
            flatten_abstract_surface_data_kind(x, city_model_arena)
        }
        AbstractFeatureKind::AbstractPointCloudKind(x) => {
            flatten_abstract_point_cloud_kind(x, city_model_arena)
        }
    }
}
