use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_texture::flatten_abstract_texture;
use crate::model::appearance::{AsAbstractTextureMut, ParameterizedTexture};
use crate::model::common::arena::InternalKey;

pub fn flatten_parameterized_texture(
    mut parameterized_texture: ParameterizedTexture,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_texture(
        parameterized_texture.abstract_texture_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(parameterized_texture.into())
        .into()
}
