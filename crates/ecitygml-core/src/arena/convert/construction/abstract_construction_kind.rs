use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::flatten_abstract_building_kind;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AbstractConstructionKind;

pub fn flatten_abstract_construction_kind(
    abstract_construction_kind: AbstractConstructionKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_construction_kind {
        AbstractConstructionKind::AbstractBuildingKind(x) => {
            flatten_abstract_building_kind(x, city_model_arena)
        }
    }
}
