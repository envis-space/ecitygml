use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::abstract_construction_surface::flatten_abstract_construction_surface;
use crate::model::common::arena::InternalKey;
use crate::model::construction::{AsAbstractConstructionSurfaceMut, CeilingSurface};

pub fn flatten_ceiling_surface(
    mut ceiling_surface: CeilingSurface,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_construction_surface(
        ceiling_surface.abstract_construction_surface_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(ceiling_surface.into())
        .into()
}
