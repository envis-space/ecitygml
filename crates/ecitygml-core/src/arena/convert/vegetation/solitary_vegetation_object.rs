use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::vegetation::abstract_vegetation_object::flatten_abstract_vegetation_object;
use crate::model::common::arena::InternalKey;
use crate::model::vegetation::{AsAbstractVegetationObjectMut, SolitaryVegetationObject};

pub fn flatten_solitary_vegetation_object(
    mut solitary_vegetation_object: SolitaryVegetationObject,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_vegetation_object(
        solitary_vegetation_object.abstract_vegetation_object_mut(),
        city_model_arena,
    );

    city_model_arena
        .insert_feature(solitary_vegetation_object.into())
        .into()
}
