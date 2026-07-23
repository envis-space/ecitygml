use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::flatten_abstract_constructive_element;
use crate::model::building::BuildingConstructiveElement;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AsAbstractConstructiveElementMut;

pub fn flatten_building_constructive_element(
    mut building_constructive_element: BuildingConstructiveElement,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_constructive_element(
        building_constructive_element.abstract_constructive_element_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(building_constructive_element.into())
        .into()
}
