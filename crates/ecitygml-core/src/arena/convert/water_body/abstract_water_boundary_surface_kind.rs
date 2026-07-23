use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::water_body::water_ground_surface::flatten_water_ground_surface;
use crate::arena::convert::water_body::water_surface::flatten_water_surface;
use crate::model::common::arena::InternalKey;
use crate::model::water_body::AbstractWaterBoundarySurfaceKind;

pub fn flatten_abstract_water_boundary_surface_kind(
    abstract_water_boundary_surface_kind: AbstractWaterBoundarySurfaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_water_boundary_surface_kind {
        AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => {
            flatten_water_ground_surface(x, city_model_arena)
        }
        AbstractWaterBoundarySurfaceKind::WaterSurface(x) => {
            flatten_water_surface(x, city_model_arena)
        }
    }
}
