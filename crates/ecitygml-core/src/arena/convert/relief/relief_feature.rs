use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_space_boundary;
use crate::arena::convert::relief::abstract_relief_component_property::flatten_abstract_relief_component_property;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractSpaceBoundaryMut;
use crate::model::relief::ReliefFeature;

pub fn flatten_relief_feature(
    mut relief_feature: ReliefFeature,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_space_boundary(
        relief_feature.abstract_space_boundary_mut(),
        city_model_arena,
    );

    for prop in relief_feature.relief_components_mut() {
        flatten_abstract_relief_component_property(prop, city_model_arena);
    }

    city_model_arena
        .insert_feature(relief_feature.into())
        .into()
}
