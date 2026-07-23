use crate::arena::CityModelArena;
use crate::model::common::LevelOfDetail;
use crate::model::core::refs::AbstractOccupiedSpaceKindRef;
use crate::model::core::{AsAbstractFeature, AsAbstractOccupiedSpace, CityModel};
use crate::model::core::{ImplicitGeometryProperty, TransformationMatrix4x4};
use egml::model::base::{AsAbstractGml, HasAssociationAttributes, Id};
use egml::model::common::ApplyTransform;
use egml::model::geometry::{AbstractGeometryKind, DirectPosition, Envelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitGeometryPlacement {
    pub geometry_id: Id,
    pub transformation_matrix: TransformationMatrix4x4,
    pub reference_point: DirectPosition,
    pub is_inline: bool,
}

impl ApplyTransform for ImplicitGeometryPlacement {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.reference_point.apply_transform(transform);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.reference_point.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.reference_point.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.reference_point.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.reference_point.apply_scale(scale);
    }
}

impl ImplicitGeometryPlacement {
    /// The world-space transform of this placement: `transformation_matrix`
    /// (rotation/scale/shear) applied first, then translated to `reference_point`.
    pub fn instance_transform(&self) -> TransformationMatrix4x4 {
        let translation = nalgebra::Matrix4::new_translation(&self.reference_point.into());
        TransformationMatrix4x4::new(translation * self.transformation_matrix.matrix())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImplicitGeometryStore {
    /// Every unique prototype geometry, indexed by the `Id` of its root geometry.
    geometry_prototypes: HashMap<Id, AbstractGeometryKind>,
    /// Every placement of a geometry prototype in an implicit geometry (either owned or only referenced),
    /// indexed by `Id` and `lod` of the feature.
    placements: HashMap<(Id, LevelOfDetail), ImplicitGeometryPlacement>,
}

impl ImplicitGeometryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_city_model(city_model: &CityModel) -> Self {
        let mut store = Self::new();

        for city_object in city_model.iter_city_objects() {
            let occupied_space: Option<AbstractOccupiedSpaceKindRef<'_>> =
                city_object.try_into().ok();
            if let Some(occupied_space) = occupied_space {
                index_occupied_space(&mut store, occupied_space);
            }
        }

        store
    }

    pub fn from_city_model_arena(city_model_arena: &CityModelArena) -> Self {
        let mut store = Self::new();

        for city_object in city_model_arena.iter_city_objects() {
            let occupied_space: Option<AbstractOccupiedSpaceKindRef<'_>> =
                city_object.try_into().ok();
            if let Some(occupied_space) = occupied_space {
                index_occupied_space(&mut store, occupied_space);
            }
        }

        store
    }

    /// Every unique prototype geometry, paired with the `Id` it's shared under.
    pub fn geometry_prototypes(&self) -> impl Iterator<Item = (&Id, &AbstractGeometryKind)> {
        self.geometry_prototypes.iter()
    }

    /// Whether `id` has an implicit geometry placement at `lod`.
    pub fn has_placement_at(&self, id: &Id, lod: LevelOfDetail) -> bool {
        self.placements.contains_key(&(id.clone(), lod))
    }

    /// Every prototype, paired with its geometry and every occurrence targeting it (feature `Id`
    /// + LOD) — the grouping GPU instancing needs: one shared mesh, many placements.
    pub fn prototypes_with_placements(
        &self,
    ) -> Vec<(
        &Id,
        &AbstractGeometryKind,
        Vec<(&Id, LevelOfDetail, &ImplicitGeometryPlacement)>,
    )> {
        let mut by_prototype: HashMap<&Id, Vec<(&Id, LevelOfDetail, &ImplicitGeometryPlacement)>> =
            HashMap::new();
        for ((feature_id, lod), placement) in &self.placements {
            by_prototype
                .entry(&placement.geometry_id)
                .or_default()
                .push((feature_id, *lod, placement));
        }

        self.geometry_prototypes
            .iter()
            .map(|(id, geometry)| {
                let placements = by_prototype.remove(id).unwrap_or_default();
                (id, geometry, placements)
            })
            .collect()
    }

    /// Drops every placement whose `reference_point` falls outside `filter_envelope`. Prototype
    /// geometries are left untouched even if they end up with no remaining placements —
    /// [`prototypes_with_placements`] already yields an empty placement list for those, which is
    /// harmless for GPU instancing (zero instances drawn).
    ///
    /// [`prototypes_with_placements`]: Self::prototypes_with_placements
    pub fn filter_placements_by_envelope(&mut self, filter_envelope: &Envelope) {
        self.placements
            .retain(|_, placement| filter_envelope.contains(&placement.reference_point));
    }
}

impl ApplyTransform for ImplicitGeometryStore {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.placements
            .iter_mut()
            .for_each(|(_, x)| x.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.placements
            .iter_mut()
            .for_each(|(_, x)| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.placements
            .iter_mut()
            .for_each(|(_, x)| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.placements
            .iter_mut()
            .for_each(|(_, x)| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.placements
            .iter_mut()
            .for_each(|(_, x)| x.apply_scale(scale));
    }
}

fn index_occupied_space(
    store: &mut ImplicitGeometryStore,
    occupied_space: AbstractOccupiedSpaceKindRef<'_>,
) {
    for (lod, prop) in [
        (
            LevelOfDetail::One,
            occupied_space.lod1_implicit_representation(),
        ),
        (
            LevelOfDetail::Two,
            occupied_space.lod2_implicit_representation(),
        ),
        (
            LevelOfDetail::Three,
            occupied_space.lod3_implicit_representation(),
        ),
    ] {
        if let Some(prop) = prop {
            index_occupied_space_implicit_representation(store, occupied_space, lod, prop);
        }
    }
}

fn index_occupied_space_implicit_representation(
    store: &mut ImplicitGeometryStore,
    occupied_space: AbstractOccupiedSpaceKindRef<'_>,
    lod: LevelOfDetail,
    property: &ImplicitGeometryProperty,
) {
    let Some(implicit_geometry) = property.object() else {
        return;
    };
    let Some(relative_geometry) = implicit_geometry.relative_geometry() else {
        return;
    };
    let Some(point) = implicit_geometry.reference_point().object() else {
        return;
    };

    let (geometry_id, owns) = match relative_geometry.object() {
        Some(geometry_prototype) => {
            let Some(id) = geometry_prototype.id() else {
                return;
            };
            store
                .geometry_prototypes
                .insert(id.clone(), geometry_prototype.to_owned());
            (id.clone(), true)
        }
        None => {
            let Some(id) = relative_geometry
                .href()
                .and_then(|href| href.local_id())
                .and_then(|local_id| Id::try_from(local_id).ok())
            else {
                return;
            };
            (id, false)
        }
    };

    store.placements.insert(
        (occupied_space.feature_id().clone(), lod),
        ImplicitGeometryPlacement {
            geometry_id,
            transformation_matrix: *implicit_geometry.transformation_matrix(),
            reference_point: *point.pos(),
            is_inline: owns,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn placement_at(x: f64, y: f64, z: f64) -> ImplicitGeometryPlacement {
        ImplicitGeometryPlacement {
            geometry_id: Id::from_hashed_string("prototype"),
            transformation_matrix: TransformationMatrix4x4::new(nalgebra::Matrix4::identity()),
            reference_point: DirectPosition::new(x, y, z).expect("finite coordinates"),
            is_inline: true,
        }
    }

    #[test]
    fn test_filter_placements_by_envelope_drops_outside_reference_points() {
        let mut store = ImplicitGeometryStore::new();
        store.placements.insert(
            (Id::from_hashed_string("inside"), LevelOfDetail::One),
            placement_at(5.0, 5.0, 5.0),
        );
        store.placements.insert(
            (Id::from_hashed_string("outside"), LevelOfDetail::One),
            placement_at(50.0, 50.0, 50.0),
        );
        store.placements.insert(
            (Id::from_hashed_string("on_boundary"), LevelOfDetail::One),
            placement_at(10.0, 10.0, 10.0),
        );

        let filter_envelope = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(10.0, 10.0, 10.0).unwrap(),
        )
        .unwrap();

        store.filter_placements_by_envelope(&filter_envelope);

        assert_eq!(store.placements.len(), 2);
        assert!(
            store
                .placements
                .contains_key(&(Id::from_hashed_string("inside"), LevelOfDetail::One))
        );
        assert!(
            store
                .placements
                .contains_key(&(Id::from_hashed_string("on_boundary"), LevelOfDetail::One))
        );
        assert!(
            !store
                .placements
                .contains_key(&(Id::from_hashed_string("outside"), LevelOfDetail::One))
        );
    }
}
