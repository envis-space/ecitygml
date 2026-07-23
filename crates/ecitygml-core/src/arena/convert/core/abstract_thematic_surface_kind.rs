use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::flatten_abstract_construction_surface_kind;
use crate::arena::convert::construction::flatten_abstract_filling_surface_kind;
use crate::arena::convert::core::closure_surface::flatten_closure_surface;
use crate::arena::convert::generics::flatten_generic_thematic_surface;
use crate::arena::convert::land_use::flatten_land_use;
use crate::arena::convert::transportation::{
    flatten_auxiliary_traffic_area, flatten_hole_surface, flatten_marking, flatten_traffic_area,
};
use crate::arena::convert::water_body::flatten_abstract_water_boundary_surface_kind;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractThematicSurfaceKind;

pub fn flatten_abstract_thematic_surface_kind(
    abstract_thematic_surface_kind: AbstractThematicSurfaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_thematic_surface_kind {
        AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => {
            flatten_auxiliary_traffic_area(x, city_model_arena)
        }
        AbstractThematicSurfaceKind::ClosureSurface(x) => {
            flatten_closure_surface(x, city_model_arena)
        }
        AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
            flatten_abstract_construction_surface_kind(x, city_model_arena)
        }
        AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
            flatten_abstract_filling_surface_kind(x, city_model_arena)
        }
        AbstractThematicSurfaceKind::GenericThematicSurface(x) => {
            flatten_generic_thematic_surface(x, city_model_arena)
        }
        AbstractThematicSurfaceKind::HoleSurface(x) => flatten_hole_surface(x, city_model_arena),
        AbstractThematicSurfaceKind::LandUse(x) => flatten_land_use(x, city_model_arena),
        AbstractThematicSurfaceKind::Marking(x) => flatten_marking(x, city_model_arena),
        AbstractThematicSurfaceKind::TrafficArea(x) => flatten_traffic_area(x, city_model_arena),
        AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
            flatten_abstract_water_boundary_surface_kind(x, city_model_arena)
        }
    }
}
