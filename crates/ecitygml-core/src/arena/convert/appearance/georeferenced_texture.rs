use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_texture::flatten_abstract_texture;
use crate::model::appearance::{AsAbstractTextureMut, GeoreferencedTexture};
use crate::model::common::arena::InternalKey;

pub fn flatten_georeferenced_texture(
    mut georeferenced_texture: GeoreferencedTexture,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_texture(
        georeferenced_texture.abstract_texture_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(georeferenced_texture.into())
        .into()
}
