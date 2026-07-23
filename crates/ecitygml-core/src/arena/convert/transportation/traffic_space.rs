use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_unoccupied_space;
use crate::arena::convert::transportation::clearance_space_property::flatten_clearance_space_property;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractUnoccupiedSpaceMut;
use crate::model::transportation::TrafficSpace;

pub fn flatten_traffic_space(
    mut traffic_space: TrafficSpace,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_unoccupied_space(
        traffic_space.abstract_unoccupied_space_mut(),
        city_model_arena,
    );

    for prop in traffic_space.clearance_spaces_mut() {
        flatten_clearance_space_property(prop, city_model_arena);
    }

    city_model_arena.insert_feature(traffic_space.into()).into()
}
