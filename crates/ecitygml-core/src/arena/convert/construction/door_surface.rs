use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_surface::flatten_abstract_filling_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractFillingSurfaceMut, DoorSurface};

pub fn flatten_door_surface(
    mut door_surface: DoorSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_filling_surface(
        door_surface.abstract_filling_surface_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(door_surface.into()).into()
}
