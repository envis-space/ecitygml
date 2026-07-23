use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_point_cloud;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractPointCloudMut;
use crate::model::point_cloud::PointCloud;

pub fn flatten_point_cloud(
    mut point_cloud: PointCloud,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_point_cloud(point_cloud.abstract_point_cloud_mut(), city_model_arena);

    city_model_arena.insert_feature(point_cloud.into()).into()
}
