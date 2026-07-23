use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::building::flatten_building;
use crate::arena::convert::building::building_part::flatten_building_part;
use crate::model::building::AbstractBuildingKind;
use crate::model::common::arena::InternalKey;

pub fn flatten_abstract_building_kind(
    abstract_building_kind: AbstractBuildingKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_building_kind {
        AbstractBuildingKind::Building(x) => flatten_building(x, city_model_arena),
        AbstractBuildingKind::BuildingPart(x) => flatten_building_part(x, city_model_arena),
    }
}
