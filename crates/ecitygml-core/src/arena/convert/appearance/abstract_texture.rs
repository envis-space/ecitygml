use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_surface_data::flatten_abstract_surface_data;
use crate::model::appearance::{AbstractTexture, AsAbstractSurfaceDataMut};

pub fn flatten_abstract_texture(
    abstract_texture: &mut AbstractTexture,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_surface_data(
        abstract_texture.abstract_surface_data_mut(),
        city_model_arena,
    )
}
