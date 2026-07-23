use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, OuterFloorSurface};

pub fn flatten_outer_floor_surface(
    mut outer_floor_surface: OuterFloorSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        outer_floor_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(outer_floor_surface.into())
        .into()
}
