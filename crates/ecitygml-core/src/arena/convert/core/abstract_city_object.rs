use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::{
    flatten_abstract_appearance_property, flatten_abstract_feature_with_lifespan,
};
use crate::model::core::{
    AbstractCityObject, AsAbstractCityObjectMut, AsAbstractFeatureWithLifespanMut,
};

pub fn flatten_abstract_city_object(
    abstract_city_object: &mut AbstractCityObject,
    city_model_arena: &mut CityModelArena,
) {
    flatten_abstract_feature_with_lifespan(
        abstract_city_object.abstract_feature_with_lifespan_mut(),
        city_model_arena,
    );

    for prop in abstract_city_object.appearances_mut() {
        flatten_abstract_appearance_property(prop, city_model_arena);
    }
}
