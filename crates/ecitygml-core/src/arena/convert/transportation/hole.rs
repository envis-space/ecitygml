use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_unoccupied_space;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractUnoccupiedSpaceMut;
use crate::model::transportation::Hole;

pub fn flatten_hole(mut hole: Hole, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_unoccupied_space(hole.abstract_unoccupied_space_mut(), city_model_arena);

    city_model_arena.insert_feature(hole.into()).into()
}
