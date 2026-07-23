use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::flatten_abstract_installation;
use crate::model::building::BuildingInstallation;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AsAbstractInstallationMut;

pub fn flatten_building_installation(
    mut building_installation: BuildingInstallation,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_installation(
        building_installation.abstract_installation_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(building_installation.into())
        .into()
}
