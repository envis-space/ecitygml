use crate::arena::city_model_arena::CityModelArena;
use crate::arena::convert::core::flatten_abstract_thematic_surface;
use crate::model::common::arena::InternalKey;
use crate::model::core::AsAbstractThematicSurfaceMut;
use crate::model::transportation::TrafficArea;

pub fn flatten_traffic_area(
    mut traffic_area: TrafficArea,
    city_model_arena: &mut CityModelArena,
) -> InternalKey {
    flatten_abstract_thematic_surface(
        traffic_area.abstract_thematic_surface_mut(),
        city_model_arena,
    );

    city_model_arena.insert_feature(traffic_area.into()).into()
}
