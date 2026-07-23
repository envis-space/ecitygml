use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::abstract_building_subdivision::flatten_abstract_building_subdivision;
use crate::model::building::AsAbstractBuildingSubdivisionMut;
use crate::model::building::Storey;
use crate::model::common::arena::InternalKey;

pub fn flatten_storey(mut storey: Storey, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_building_subdivision(
        storey.abstract_building_subdivision_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(storey.into()).into()
}
