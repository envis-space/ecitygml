use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::{
    flatten_abstract_city_object, flatten_abstract_space_boundary_property,
};
use crate::model::core::{AbstractSpace, AsAbstractCityObjectMut, AsAbstractSpaceMut};

pub fn flatten_abstract_space(
    abstract_space: &mut AbstractSpace,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_city_object(abstract_space.abstract_city_object_mut(), city_model_arena);

    for prop in abstract_space.boundaries_mut() {
        flatten_abstract_space_boundary_property(prop, city_model_arena);
    }
}
