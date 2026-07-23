use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::transportation::HoleSurface;

pub fn flatten_hole_surface(
    mut hole_surface: HoleSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_thematic_surface(
        hole_surface.abstract_thematic_surface_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(hole_surface.into()).into()
}
