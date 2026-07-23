use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::transportation::abstract_transportation_space::flatten_abstract_transportation_space;
use crate::model::common::arena::InternalKey;
use crate::model::transportation::{AsAbstractTransportationSpaceMut, Square};

pub fn flatten_square(mut square: Square, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_transportation_space(
        square.abstract_transportation_space_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(square.into()).into()
}
