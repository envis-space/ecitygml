use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::water_body::AbstractWaterBoundarySurface;

pub fn flatten_abstract_water_boundary_surface(
    abstract_water_boundary_surface: &mut AbstractWaterBoundarySurface,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_thematic_surface(
        abstract_water_boundary_surface.abstract_thematic_surface_mut(),
        city_model_arena,
    )
}
