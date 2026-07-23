use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building_constructive_element_property::flatten_building_constructive_element_property;
use crate::arena::convert::core::flatten_abstract_logical_space;
use crate::model::building::{AbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut};
use crate::model::core::AsAbstractLogicalSpaceMut;

pub fn flatten_abstract_building_subdivision(
    abstract_building_subdivision: &mut AbstractBuildingSubdivision,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_logical_space(
        abstract_building_subdivision.abstract_logical_space_mut(),
        city_model_arena,
    );

    for prop in abstract_building_subdivision.building_constructive_elements_mut() {
        flatten_building_constructive_element_property(prop, city_model_arena);
    }
}
