use crate::Error;
use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_appearance_property::flatten_abstract_appearance_property;
use crate::arena::convert::core::abstract_city_object_property::flatten_abstract_city_object_property;
use crate::arena::convert::core::abstract_feature_with_lifespan::flatten_abstract_feature_with_lifespan;
use crate::model::common::arena::InternalKey;
use crate::model::core::CityModel;
use egml::model::base::Id;

pub fn flatten_city_model(
    mut city_model: CityModel,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_feature_with_lifespan(
        &mut city_model.abstract_feature_with_lifespan,
        city_model_arena,
    );

    for city_object_property in city_model.city_object_members_mut() {
        flatten_abstract_city_object_property(city_object_property, city_model_arena);
    }

    for appearance_property in city_model.appearance_members_mut() {
        flatten_abstract_appearance_property(appearance_property, city_model_arena);
    }

    city_model_arena.insert_feature(city_model.into()).into()
}

pub fn unflatten_city_model_arena(_city_model_arena: CityModelArena) -> Result<CityModel, Error> {
    Ok(CityModel::new(Id::generate_uuid_v4()))
}
