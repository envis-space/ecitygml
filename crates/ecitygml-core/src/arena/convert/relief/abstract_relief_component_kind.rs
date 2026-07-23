use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::relief::tin_relief::flatten_tin_relief;
use crate::model::common::arena::InternalKey;
use crate::model::relief::AbstractReliefComponentKind;

pub fn flatten_abstract_relief_component_kind(
    abstract_relief_component_kind: AbstractReliefComponentKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_relief_component_kind {
        AbstractReliefComponentKind::TinRelief(x) => flatten_tin_relief(x, city_model_arena),
    }
}
