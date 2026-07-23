use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building_unit::flatten_building_unit;
use crate::arena::convert::building::storey::flatten_storey;
use crate::model::building::AbstractBuildingSubdivisionKind;
use crate::model::common::arena::InternalKey;

pub fn flatten_abstract_building_subdivision_kind(
    abstract_building_subdivision_kind: AbstractBuildingSubdivisionKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_building_subdivision_kind {
        AbstractBuildingSubdivisionKind::BuildingUnit(x) => {
            flatten_building_unit(x, city_model_arena)
        }
        AbstractBuildingSubdivisionKind::Storey(x) => flatten_storey(x, city_model_arena),
    }
}
