use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::transportation::Marking;

pub fn flatten_marking(mut marking: Marking, city_model_arena: &mut CityModelArena) -> InternalKey {
    flatten_abstract_thematic_surface(marking.abstract_thematic_surface_mut(), city_model_arena);

    city_model_arena.insert_feature(marking.into()).into()
}
