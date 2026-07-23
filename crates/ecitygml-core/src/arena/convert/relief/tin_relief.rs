use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::relief::abstract_relief_component::flatten_abstract_relief_component;
use crate::model::common::arena::InternalKey;
use crate::model::relief::{AsAbstractReliefComponentMut, TinRelief};

pub fn flatten_tin_relief(
    mut tin_relief: TinRelief,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_relief_component(tin_relief.abstract_relief_component_mut(), city_model_arena);

    city_model_arena.insert_feature(tin_relief.into()).into()
}
