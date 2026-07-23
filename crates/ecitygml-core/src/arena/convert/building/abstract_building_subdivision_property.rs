use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::building::abstract_building_subdivision_kind::flatten_abstract_building_subdivision_kind;
use crate::model::building::AbstractBuildingSubdivisionProperty;
use crate::model::common::arena::HasArenaPropertiesMut;

pub fn flatten_abstract_building_subdivision_property(
    abstract_building_subdivision_property: &mut AbstractBuildingSubdivisionProperty,
    city_model_arena: &mut CityModelArena,
) {
    let Some(object) = abstract_building_subdivision_property.take_object() else {
        return;
    };

    let internal_key = flatten_abstract_building_subdivision_kind(object, city_model_arena);
    abstract_building_subdivision_property.set_key(internal_key);
}
