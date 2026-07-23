use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::ceiling_surface::flatten_ceiling_surface;
use crate::arena::convert::construction::floor_surface::flatten_floor_surface;
use crate::arena::convert::construction::ground_surface::flatten_ground_surface;
use crate::arena::convert::construction::interior_wall_surface::flatten_interior_wall_surface;
use crate::arena::convert::construction::outer_ceiling_surface::flatten_outer_ceiling_surface;
use crate::arena::convert::construction::outer_floor_surface::flatten_outer_floor_surface;
use crate::arena::convert::construction::roof_surface::flatten_roof_surface;
use crate::arena::convert::construction::wall_surface::flatten_wall_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AbstractConstructionSurfaceKind;

pub fn flatten_abstract_construction_surface_kind(
    abstract_construction_surface_kind: AbstractConstructionSurfaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_construction_surface_kind {
        AbstractConstructionSurfaceKind::CeilingSurface(x) => {
            flatten_ceiling_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::FloorSurface(x) => {
            flatten_floor_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::GroundSurface(x) => {
            flatten_ground_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::InteriorWallSurface(x) => {
            flatten_interior_wall_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => {
            flatten_outer_ceiling_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::OuterFloorSurface(x) => {
            flatten_outer_floor_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::RoofSurface(x) => {
            flatten_roof_surface(x, city_model_arena)
        }
        AbstractConstructionSurfaceKind::WallSurface(x) => {
            flatten_wall_surface(x, city_model_arena)
        }
    }
}
