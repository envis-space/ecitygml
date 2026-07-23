use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_point_cloud_property;
use crate::arena::convert::core::flatten_abstract_space;
use crate::model::core::{AbstractPhysicalSpace, AsAbstractPhysicalSpaceMut, AsAbstractSpaceMut};

pub fn flatten_abstract_physical_space(
    abstract_physical_space: &mut AbstractPhysicalSpace,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_space(
        abstract_physical_space.abstract_space_mut(),
        city_model_arena,
    );

    if let Some(prop) = abstract_physical_space.point_cloud_mut() {
        flatten_abstract_point_cloud_property(prop, city_model_arena);
    }
}
