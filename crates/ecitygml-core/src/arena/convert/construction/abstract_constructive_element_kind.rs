use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::flatten_building_constructive_element;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AbstractConstructiveElementKind;

pub fn flatten_abstract_constructive_element_kind(
    abstract_constructive_element_kind: AbstractConstructiveElementKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_constructive_element_kind {
        AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
            flatten_building_constructive_element(x, city_model_arena)
        }
    }
}
