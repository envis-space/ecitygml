use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::abstract_point_cloud_kind::flatten_abstract_point_cloud_kind;
use crate::model::common::arena::HasArenaPropertiesMut;
use crate::model::core::AbstractPointCloudProperty;

pub fn flatten_abstract_point_cloud_property(
    abstract_point_cloud_property: &mut AbstractPointCloudProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_point_cloud_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_point_cloud_kind(object, city_model_arena);
    abstract_point_cloud_property.set_key(internal_key);
}
