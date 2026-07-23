use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_unoccupied_space;
use crate::model::building::BuildingRoom;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractUnoccupiedSpaceMut;

pub fn flatten_building_room(
    mut building_room: BuildingRoom,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_unoccupied_space(
        building_room.abstract_unoccupied_space_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(building_room.into()).into()
}
