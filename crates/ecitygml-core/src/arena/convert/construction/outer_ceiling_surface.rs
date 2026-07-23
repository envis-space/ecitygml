use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, OuterCeilingSurface};

pub fn flatten_outer_ceiling_surface(
    mut outer_ceiling_surface: OuterCeilingSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        outer_ceiling_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(outer_ceiling_surface.into())
        .into()
}
