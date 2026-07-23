use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::vegetation::plant_cover::flatten_plant_cover;
use crate::arena::convert::vegetation::solitary_vegetation_object::flatten_solitary_vegetation_object;
use crate::model::common::arena::InternalKey;
use crate::model::vegetation::AbstractVegetationObjectKind;

pub fn flatten_abstract_vegetation_object_kind(
    abstract_vegetation_object_kind: AbstractVegetationObjectKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_vegetation_object_kind {
        AbstractVegetationObjectKind::PlantCover(x) => flatten_plant_cover(x, city_model_arena),
        AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
            flatten_solitary_vegetation_object(x, city_model_arena)
        }
    }
}
