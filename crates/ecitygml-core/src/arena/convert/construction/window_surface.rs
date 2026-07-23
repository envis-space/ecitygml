use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_filling_surface::flatten_abstract_filling_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractFillingSurfaceMut, WindowSurface};

pub fn flatten_window_surface(
    mut window_surface: WindowSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_filling_surface(
        window_surface.abstract_filling_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(window_surface.into())
        .into()
}
