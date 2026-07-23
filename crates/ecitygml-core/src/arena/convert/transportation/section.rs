use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::abstract_transportation_space::flatten_abstract_transportation_space;
use crate::model::common::arena::InternalKey;
use crate::model::transportation::{AsAbstractTransportationSpaceMut, Section};

pub fn flatten_section(mut section: Section, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_transportation_space(
        section.abstract_transportation_space_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(section.into()).into()
}
