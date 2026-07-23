use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_feature::flatten_abstract_feature;
use crate::model::core::{AbstractPointCloud, AsAbstractFeatureMut};

pub fn flatten_abstract_point_cloud(
    abstract_point_cloud: &mut AbstractPointCloud,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_feature(
        abstract_point_cloud.abstract_feature_mut(),
        city_model_arena,
    )
}
