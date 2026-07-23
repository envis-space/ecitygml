use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::construction::door::flatten_door;
use crate::arena::convert::construction::window::flatten_window;
use crate::model::common::arena::InternalKey;
use crate::model::construction::AbstractFillingElementKind;

pub fn flatten_abstract_filling_element_kind(
    abstract_filling_element_kind: AbstractFillingElementKind,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    match abstract_filling_element_kind {
        AbstractFillingElementKind::Door(x) => flatten_door(x, city_model_arena),
        AbstractFillingElementKind::Window(x) => flatten_window(x, city_model_arena),
    }
}
