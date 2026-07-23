use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, GroundSurface};

pub fn flatten_ground_surface(
    mut ground_surface: GroundSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        ground_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(ground_surface.into())
        .into()
}
