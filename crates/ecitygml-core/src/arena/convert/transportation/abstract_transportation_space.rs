use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_unoccupied_space;
use crate::arena::convert::transportation::auxiliary_traffic_space_property::flatten_auxiliary_traffic_space_property;
use crate::arena::convert::transportation::hole_property::flatten_hole_property;
use crate::arena::convert::transportation::marking_property::flatten_marking_property;
use crate::arena::convert::transportation::traffic_space_property::flatten_traffic_space_property;
use crate::model::core::AsAbstractUnoccupiedSpaceMut;
use crate::model::transportation::{AbstractTransportationSpace, AsAbstractTransportationSpaceMut};

pub fn flatten_abstract_transportation_space(
    abstract_transportation_space: &mut AbstractTransportationSpace,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_unoccupied_space(
        abstract_transportation_space.abstract_unoccupied_space_mut(),
        city_model_arena,
    );

    for prop in abstract_transportation_space.traffic_spaces_mut() {
        flatten_traffic_space_property(prop, city_model_arena);
    }
    for prop in abstract_transportation_space.auxiliary_traffic_spaces_mut() {
        flatten_auxiliary_traffic_space_property(prop, city_model_arena);
    }
    for prop in abstract_transportation_space.markings_mut() {
        flatten_marking_property(prop, city_model_arena);
    }
    for prop in abstract_transportation_space.holes_mut() {
        flatten_hole_property(prop, city_model_arena);
    }
}
