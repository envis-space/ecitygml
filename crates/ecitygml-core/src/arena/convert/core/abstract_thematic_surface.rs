use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_point_cloud_property::flatten_abstract_point_cloud_property;
use crate::arena::convert::core::abstract_space_boundary::flatten_abstract_space_boundary;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractSpaceBoundaryMut, AsAbstractThematicSurfaceMut,
};

pub fn flatten_abstract_thematic_surface(
    abstract_thematic_surface: &mut AbstractThematicSurface,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_space_boundary(
        abstract_thematic_surface.abstract_space_boundary_mut(),
        city_model_arena,
    );

    if let Some(prop) = abstract_thematic_surface.point_cloud_mut() {
        flatten_abstract_point_cloud_property(prop, city_model_arena);
    }
}
