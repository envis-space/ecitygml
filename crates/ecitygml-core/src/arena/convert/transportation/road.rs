use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::abstract_transportation_space::flatten_abstract_transportation_space;
use crate::arena::convert::transportation::intersection_property::flatten_intersection_property;
use crate::arena::convert::transportation::section_property::flatten_section_property;
use crate::model::common::arena::InternalKey;
use crate::model::transportation::{AsAbstractTransportationSpaceMut, Road};

pub fn flatten_road(mut road: Road, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_transportation_space(
        road.abstract_transportation_space_mut(),
        city_model_arena,
    );

    for prop in road.sections_mut() {
        flatten_section_property(prop, city_model_arena);
    }
    for prop in road.intersections_mut() {
        flatten_intersection_property(prop, city_model_arena);
    }

    city_model_arena.insert_feature(road.into()).into()
}
