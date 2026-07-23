use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, InteriorWallSurface};

pub fn flatten_interior_wall_surface(
    mut interior_wall_surface: InteriorWallSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        interior_wall_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(interior_wall_surface.into())
        .into()
}
