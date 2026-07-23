use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::point_cloud::flatten_point_cloud;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractPointCloudKind;

pub fn flatten_abstract_point_cloud_kind(
    abstract_point_cloud_kind: AbstractPointCloudKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_point_cloud_kind {
        AbstractPointCloudKind::PointCloud(x) => flatten_point_cloud(x, city_model_arena),
    }
}
