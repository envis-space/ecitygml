use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::georeferenced_texture::flatten_georeferenced_texture;
use crate::arena::convert::appearance::parameterized_texture::flatten_parameterized_texture;
use crate::model::appearance::AbstractTextureKind;
use crate::model::common::arena::InternalKey;

pub fn flatten_abstract_texture_kind(
    abstract_texture_kind: AbstractTextureKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_texture_kind {
        AbstractTextureKind::GeoreferencedTexture(x) => {
            flatten_georeferenced_texture(x, city_model_arena)
        }
        AbstractTextureKind::ParameterizedTexture(x) => {
            flatten_parameterized_texture(x, city_model_arena)
        }
    }
}
