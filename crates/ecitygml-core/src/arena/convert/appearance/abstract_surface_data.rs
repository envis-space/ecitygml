use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_feature;
use crate::model::appearance::AbstractSurfaceData;
use crate::model::core::AsAbstractFeatureMut;

pub fn flatten_abstract_surface_data(
    abstract_surface_data: &mut AbstractSurfaceData,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_feature(
        abstract_surface_data.abstract_feature_mut(),
        city_model_arena,
    )
}
