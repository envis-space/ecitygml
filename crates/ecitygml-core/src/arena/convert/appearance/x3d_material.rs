use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_surface_data::flatten_abstract_surface_data;
use crate::model::appearance::{AsAbstractSurfaceDataMut, X3DMaterial};
use crate::model::common::arena::InternalKey;

pub fn flatten_x3d_material(
    mut x3d_material: X3DMaterial,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_surface_data(x3d_material.abstract_surface_data_mut(), city_model_arena);

    city_model_arena.insert_feature(x3d_material.into()).into()
}
