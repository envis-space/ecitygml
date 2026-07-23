use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_surface_property::flatten_abstract_filling_surface_property;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::construction::{AbstractConstructionSurface, AsAbstractConstructionSurfaceMut};
use crate::model::core::AsAbstractThematicSurfaceMut;

pub fn flatten_abstract_construction_surface(
    abstract_construction_surface: &mut AbstractConstructionSurface,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_thematic_surface(
        abstract_construction_surface.abstract_thematic_surface_mut(),
        city_model_arena,
    );

    for prop in abstract_construction_surface.filling_surfaces_mut() {
        flatten_abstract_filling_surface_property(prop, city_model_arena);
    }
}
