use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::city_furniture::flatten_city_furniture;
use crate::arena::convert::construction::flatten_abstract_construction_kind;
use crate::arena::convert::construction::flatten_abstract_constructive_element_kind;
use crate::arena::convert::construction::flatten_abstract_filling_element_kind;
use crate::arena::convert::construction::flatten_abstract_installation_kind;
use crate::arena::convert::generics::flatten_generic_occupied_space;
use crate::arena::convert::vegetation::flatten_abstract_vegetation_object_kind;
use crate::arena::convert::water_body::flatten_water_body;
use crate::model::common::arena::InternalKey;
use crate::model::core::AbstractOccupiedSpaceKind;

pub fn flatten_abstract_occupied_space_kind(
    abstract_occupied_space_kind: AbstractOccupiedSpaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_occupied_space_kind {
        AbstractOccupiedSpaceKind::CityFurniture(x) => flatten_city_furniture(x, city_model_arena),
        AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => {
            flatten_abstract_construction_kind(x, city_model_arena)
        }
        AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
            flatten_abstract_constructive_element_kind(x, city_model_arena)
        }
        AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => {
            flatten_abstract_filling_element_kind(x, city_model_arena)
        }
        AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => {
            flatten_generic_occupied_space(x, city_model_arena)
        }
        AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => {
            flatten_abstract_installation_kind(x, city_model_arena)
        }
        AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => {
            flatten_abstract_vegetation_object_kind(x, city_model_arena)
        }
        AbstractOccupiedSpaceKind::WaterBody(x) => flatten_water_body(x, city_model_arena),
    }
}
