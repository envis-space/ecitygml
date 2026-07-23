use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::construction::AbstractFillingSurface;
use crate::model::core::AsAbstractThematicSurfaceMut;

pub fn flatten_abstract_filling_surface(
    abstract_filling_surface: &mut AbstractFillingSurface,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_thematic_surface(
        abstract_filling_surface.abstract_thematic_surface_mut(),
        city_model_arena,
    )
}
