use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::vegetation::abstract_vegetation_object::flatten_abstract_vegetation_object;
use crate::model::common::arena::InternalKey;
use crate::model::vegetation::{AsAbstractVegetationObjectMut, PlantCover};

pub fn flatten_plant_cover(
    mut plant_cover: PlantCover,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_vegetation_object(
        plant_cover.abstract_vegetation_object_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(plant_cover.into()).into()
}
