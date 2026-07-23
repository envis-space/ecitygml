use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::construction::AbstractConstruction;
use crate::model::core::AsAbstractOccupiedSpaceMut;

pub fn flatten_abstract_construction(
    abstract_construction: &mut AbstractConstruction,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_occupied_space(
        abstract_construction.abstract_occupied_space_mut(),
        city_model_arena,
    );
}
