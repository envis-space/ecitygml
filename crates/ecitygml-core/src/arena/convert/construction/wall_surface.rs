use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, WallSurface};

pub fn flatten_wall_surface(
    mut wall_surface: WallSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        wall_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(wall_surface.into()).into()
}
