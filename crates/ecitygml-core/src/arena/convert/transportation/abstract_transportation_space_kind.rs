use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::intersection::flatten_intersection;
use crate::arena::convert::transportation::railway::flatten_railway;
use crate::arena::convert::transportation::road::flatten_road;
use crate::arena::convert::transportation::section::flatten_section;
use crate::arena::convert::transportation::square::flatten_square;
use crate::arena::convert::transportation::track::flatten_track;
use crate::arena::convert::transportation::waterway::flatten_waterway;
use crate::model::common::arena::InternalKey;
use crate::model::transportation::AbstractTransportationSpaceKind;

pub fn flatten_abstract_transportation_space_kind(
    abstract_transportation_space_kind: AbstractTransportationSpaceKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_transportation_space_kind {
        AbstractTransportationSpaceKind::Section(x) => flatten_section(x, city_model_arena),
        AbstractTransportationSpaceKind::Intersection(x) => {
            flatten_intersection(x, city_model_arena)
        }
        AbstractTransportationSpaceKind::Road(x) => flatten_road(x, city_model_arena),
        AbstractTransportationSpaceKind::Track(x) => flatten_track(x, city_model_arena),
        AbstractTransportationSpaceKind::Railway(x) => flatten_railway(x, city_model_arena),
        AbstractTransportationSpaceKind::Waterway(x) => flatten_waterway(x, city_model_arena),
        AbstractTransportationSpaceKind::Square(x) => flatten_square(x, city_model_arena),
    }
}
