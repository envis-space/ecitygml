use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_occupied_space;
use crate::model::construction::AbstractInstallation;
use crate::model::core::AsAbstractOccupiedSpaceMut;

pub fn flatten_abstract_installation(
    abstract_installation: &mut AbstractInstallation,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_occupied_space(
        abstract_installation.abstract_occupied_space_mut(),
        city_model_arena,
    )
}
