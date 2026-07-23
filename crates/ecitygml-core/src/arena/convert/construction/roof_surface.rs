use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, RoofSurface};

pub fn flatten_roof_surface(
    mut roof_surface: RoofSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        roof_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(roof_surface.into()).into()
}
