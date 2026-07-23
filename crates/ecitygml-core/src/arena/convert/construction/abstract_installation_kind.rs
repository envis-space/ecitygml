use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::flatten_building_installation;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AbstractInstallationKind;

pub fn flatten_abstract_installation_kind(
    abstract_installation_kind: AbstractInstallationKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_installation_kind {
        AbstractInstallationKind::BuildingInstallation(x) => {
            flatten_building_installation(x, city_model_arena)
        }
    }
}
