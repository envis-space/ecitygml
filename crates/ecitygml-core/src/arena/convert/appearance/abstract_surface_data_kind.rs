use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::appearance::abstract_texture_kind::flatten_abstract_texture_kind;
use crate::arena::convert::appearance::x3d_material::flatten_x3d_material;
use crate::model::appearance::AbstractSurfaceDataKind;
use crate::model::common::arena::InternalKey;

pub fn flatten_abstract_surface_data_kind(
    abstract_surface_data_kind: AbstractSurfaceDataKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_surface_data_kind {
        AbstractSurfaceDataKind::AbstractTextureKind(x) => {
            flatten_abstract_texture_kind(x, city_model_arena)
        }
        AbstractSurfaceDataKind::X3DMaterial(x) => flatten_x3d_material(x, city_model_arena),
    }
}
