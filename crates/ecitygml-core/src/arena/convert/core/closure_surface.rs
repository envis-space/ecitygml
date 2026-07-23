use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_thematic_surface::flatten_abstract_thematic_surface;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::core::ClosureSurface;

pub fn flatten_closure_surface(
    mut closure_surface: ClosureSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_thematic_surface(
        closure_surface.abstract_thematic_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(closure_surface.into())
        .into()
}
