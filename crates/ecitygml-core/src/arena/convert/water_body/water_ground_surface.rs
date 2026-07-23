use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::water_body::abstract_water_boundary_surface::flatten_abstract_water_boundary_surface;
use crate::model::common::arena::InternalKey;
use crate::model::water_body::{AsAbstractWaterBoundarySurfaceMut, WaterGroundSurface};

pub fn flatten_water_ground_surface(
    mut water_ground_surface: WaterGroundSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_water_boundary_surface(
        water_ground_surface.abstract_water_boundary_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(water_ground_surface.into())
        .into()
}
