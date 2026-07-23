use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::generics::GenericThematicSurface;

pub fn flatten_generic_thematic_surface(
    mut generic_thematic_surface: GenericThematicSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_thematic_surface(
        generic_thematic_surface.abstract_thematic_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(generic_thematic_surface.into())
        .into()
}
