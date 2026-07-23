use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::land_use::LandUse;

pub fn flatten_land_use(
    mut land_use: LandUse,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_thematic_surface(land_use.abstract_thematic_surface_mut(), city_model_arena);

    city_model_arena.insert_feature(land_use.into()).into()
}
