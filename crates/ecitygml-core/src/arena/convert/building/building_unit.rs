use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::abstract_building_subdivision::flatten_abstract_building_subdivision;
use crate::model::building::AsAbstractBuildingSubdivisionMut;
use crate::model::building::BuildingUnit;
use crate::model::common::arena::InternalKey;

pub fn flatten_building_unit(
    mut building_unit: BuildingUnit,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_building_subdivision(
        building_unit.abstract_building_subdivision_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(building_unit.into()).into()
}
