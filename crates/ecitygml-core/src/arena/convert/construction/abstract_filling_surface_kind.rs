use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::door_surface::flatten_door_surface;
use crate::arena::convert::construction::window_surface::flatten_window_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AbstractFillingSurfaceKind;

pub fn flatten_abstract_filling_surface_kind(
    abstract_filling_surface_kind: AbstractFillingSurfaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_filling_surface_kind {
        AbstractFillingSurfaceKind::DoorSurface(x) => flatten_door_surface(x, city_model_arena),
        AbstractFillingSurfaceKind::WindowSurface(x) => flatten_window_surface(x, city_model_arena),
    }
}
