use crate::arena::CityModelArena;
use crate::model::core::refs::{AbstractCityObjectKindRef, AbstractFeatureKindRef};
use bvh::aabb::{Aabb, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::Bvh;
use egml::model::geometry::Envelope;
use nalgebra::Point3;

struct SpatialEntry {
    index: usize,
    aabb: Aabb<f64, 3>,
    node_index: usize,
}

impl SpatialEntry {
    fn from_object(index: usize, _obj: AbstractFeatureKindRef) -> Option<Self> {
        /*let env = obj.bounded_by().and_then(|x| x.envelope())?;


        */
        let env = Envelope::default();
        Some(Self {
            index,
            aabb: envelope_to_aabb(&env),
            node_index: 0,
        })
    }
}

impl Bounded<f64, 3> for SpatialEntry {
    fn aabb(&self) -> Aabb<f64, 3> {
        self.aabb
    }
}

impl BHShape<f64, 3> for SpatialEntry {
    fn bh_node_index(&self) -> usize {
        self.node_index
    }

    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }
}

pub struct BvhIndex {
    bvh: Bvh<f64, 3>,
    entries: Vec<SpatialEntry>,
    city_model_arena: CityModelArena,
}

impl BvhIndex {
    pub fn from_geometry_store(city_model_arena: CityModelArena) -> Self {
        let mut entries: Vec<SpatialEntry> = city_model_arena
            .iter_features()
            .enumerate()
            .filter_map(|(i, obj)| SpatialEntry::from_object(i, obj))
            .collect();

        let bvh = Bvh::build(&mut entries);

        Self {
            bvh,
            entries,
            city_model_arena,
        }
    }

    pub fn query_envelope(&self, envelope: &Envelope) -> Vec<&AbstractCityObjectKindRef<'_>> {
        let _query_aabb = envelope_to_aabb(envelope);

        /*  self.bvh
        .traverse_iterator(&query_aabb, &self.entries)
        .filter_map(|entry| self.city_model_arena.get_by_index(entry.index))
        .collect()*/

        Vec::new()
    }
}

fn envelope_to_aabb(envelope: &Envelope) -> Aabb<f64, 3> {
    let min = Point3::new(
        envelope.lower_corner().x(),
        envelope.lower_corner().y(),
        envelope.lower_corner().z(),
    );
    let max = Point3::new(
        envelope.upper_corner().x(),
        envelope.upper_corner().y(),
        envelope.upper_corner().z(),
    );
    Aabb::with_bounds(min, max)
}
