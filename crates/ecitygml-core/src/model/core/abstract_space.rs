use crate::model::common::{ForEachFeatureMut, IterFeatures, LevelOfDetail};
use crate::model::core::AsAbstractCityObject;
use crate::model::core::abstract_space_boundary_property::AbstractSpaceBoundaryProperty;
use crate::model::core::enums::SpaceType;
use crate::model::core::qualified_area::QualifiedArea;
use crate::model::core::qualified_volume::QualifiedVolume;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractCityObject, AsAbstractCityObjectMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::{MultiCurveProperty, MultiSurfaceProperty};
use egml::model::geometry::primitives::SolidProperty;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSpace {
    pub(crate) abstract_city_object: AbstractCityObject,
    space_type: Option<SpaceType>,
    volumes: Vec<QualifiedVolume>,
    areas: Vec<QualifiedArea>,
    lod1_solid: Option<SolidProperty>,
    lod2_solid: Option<SolidProperty>,
    lod3_solid: Option<SolidProperty>,
    lod0_multi_surface: Option<MultiSurfaceProperty>,
    lod2_multi_surface: Option<MultiSurfaceProperty>,
    lod3_multi_surface: Option<MultiSurfaceProperty>,
    lod0_multi_curve: Option<MultiCurveProperty>,
    lod2_multi_curve: Option<MultiCurveProperty>,
    lod3_multi_curve: Option<MultiCurveProperty>,
    boundaries: Vec<AbstractSpaceBoundaryProperty>,
}

impl AbstractSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_city_object(AbstractCityObject::new(id))
    }

    pub fn from_abstract_city_object(abstract_city_object: AbstractCityObject) -> Self {
        Self {
            abstract_city_object,
            space_type: None,
            volumes: Vec::new(),
            areas: Vec::new(),
            lod1_solid: None,
            lod2_solid: None,
            lod3_solid: None,
            lod0_multi_surface: None,
            lod2_multi_surface: None,
            lod3_multi_surface: None,
            lod0_multi_curve: None,
            lod2_multi_curve: None,
            lod3_multi_curve: None,
            boundaries: Vec::new(),
        }
    }
}

pub trait AsAbstractSpace: AsAbstractCityObject {
    fn abstract_space(&self) -> &AbstractSpace;

    fn space_type(&self) -> Option<SpaceType> {
        self.abstract_space().space_type
    }

    fn volumes(&self) -> &[QualifiedVolume] {
        self.abstract_space().volumes.as_ref()
    }

    fn areas(&self) -> &[QualifiedArea] {
        self.abstract_space().areas.as_ref()
    }

    fn lod1_solid(&self) -> Option<&SolidProperty> {
        self.abstract_space().lod1_solid.as_ref()
    }

    fn lod2_solid(&self) -> Option<&SolidProperty> {
        self.abstract_space().lod2_solid.as_ref()
    }

    fn lod3_solid(&self) -> Option<&SolidProperty> {
        self.abstract_space().lod3_solid.as_ref()
    }

    fn solids_by_lod(&self) -> HashMap<LevelOfDetail, &SolidProperty> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod1_solid() {
            map.insert(LevelOfDetail::One, x);
        }
        if let Some(x) = self.lod2_solid() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_solid() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn lod0_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_space().lod0_multi_surface.as_ref()
    }

    fn lod2_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_space().lod2_multi_surface.as_ref()
    }

    fn lod3_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_space().lod3_multi_surface.as_ref()
    }

    fn multi_surfaces_by_lod(&self) -> HashMap<LevelOfDetail, &MultiSurfaceProperty> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod0_multi_surface() {
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod2_multi_surface() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_multi_surface() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn lod0_multi_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_space().lod0_multi_curve.as_ref()
    }

    fn lod2_multi_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_space().lod2_multi_curve.as_ref()
    }

    fn lod3_multi_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_space().lod3_multi_curve.as_ref()
    }

    fn multi_curves_by_lod(&self) -> HashMap<LevelOfDetail, &MultiCurveProperty> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod0_multi_curve() {
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod2_multi_curve() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_multi_curve() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn boundaries(&self) -> &[AbstractSpaceBoundaryProperty] {
        self.abstract_space().boundaries.as_ref()
    }
}

pub trait AsAbstractSpaceMut: AsAbstractCityObjectMut + AsAbstractSpace {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace;

    fn set_space_type(&mut self, value: Option<SpaceType>) {
        self.abstract_space_mut().space_type = value;
    }

    fn volumes_mut(&mut self) -> &mut Vec<QualifiedVolume> {
        &mut self.abstract_space_mut().volumes
    }

    fn set_volumes(&mut self, values: Vec<QualifiedVolume>) {
        self.abstract_space_mut().volumes = values;
    }

    fn push_volume(&mut self, volume: QualifiedVolume) {
        self.abstract_space_mut().volumes.push(volume);
    }

    fn extend_volumes(&mut self, volumes: impl IntoIterator<Item = QualifiedVolume>) {
        self.abstract_space_mut().volumes.extend(volumes);
    }

    fn areas_mut(&mut self) -> &mut Vec<QualifiedArea> {
        &mut self.abstract_space_mut().areas
    }

    fn set_areas(&mut self, values: Vec<QualifiedArea>) {
        self.abstract_space_mut().areas = values;
    }

    fn push_area(&mut self, area: QualifiedArea) {
        self.abstract_space_mut().areas.push(area);
    }

    fn extend_areas(&mut self, areas: impl IntoIterator<Item = QualifiedArea>) {
        self.abstract_space_mut().areas.extend(areas);
    }

    fn set_lod1_solid(&mut self, value: Option<SolidProperty>) {
        self.abstract_space_mut().lod1_solid = value;
    }

    fn set_lod2_solid(&mut self, value: Option<SolidProperty>) {
        self.abstract_space_mut().lod2_solid = value;
    }

    fn set_lod3_solid(&mut self, value: Option<SolidProperty>) {
        self.abstract_space_mut().lod3_solid = value;
    }

    fn set_lod0_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_space_mut().lod0_multi_surface = value;
    }

    fn set_lod2_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_space_mut().lod2_multi_surface = value;
    }

    fn set_lod3_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_space_mut().lod3_multi_surface = value;
    }

    fn set_lod0_multi_curve(&mut self, value: Option<MultiCurveProperty>) {
        self.abstract_space_mut().lod0_multi_curve = value;
    }

    fn set_lod2_multi_curve(&mut self, value: Option<MultiCurveProperty>) {
        self.abstract_space_mut().lod2_multi_curve = value;
    }

    fn set_lod3_multi_curve(&mut self, value: Option<MultiCurveProperty>) {
        self.abstract_space_mut().lod3_multi_curve = value;
    }

    fn lod1_solid_mut(&mut self) -> Option<&mut SolidProperty> {
        self.abstract_space_mut().lod1_solid.as_mut()
    }

    fn lod2_solid_mut(&mut self) -> Option<&mut SolidProperty> {
        self.abstract_space_mut().lod2_solid.as_mut()
    }

    fn lod3_solid_mut(&mut self) -> Option<&mut SolidProperty> {
        self.abstract_space_mut().lod3_solid.as_mut()
    }

    fn lod0_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_space_mut().lod0_multi_surface.as_mut()
    }

    fn lod2_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_space_mut().lod2_multi_surface.as_mut()
    }

    fn lod3_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_space_mut().lod3_multi_surface.as_mut()
    }

    fn lod0_multi_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_space_mut().lod0_multi_curve.as_mut()
    }

    fn lod2_multi_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_space_mut().lod2_multi_curve.as_mut()
    }

    fn lod3_multi_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_space_mut().lod3_multi_curve.as_mut()
    }

    fn boundaries_mut(&mut self) -> &mut Vec<AbstractSpaceBoundaryProperty> {
        &mut self.abstract_space_mut().boundaries
    }

    fn set_boundaries(&mut self, values: Vec<AbstractSpaceBoundaryProperty>) {
        self.abstract_space_mut().boundaries = values;
    }

    fn push_boundary(&mut self, boundary: AbstractSpaceBoundaryProperty) {
        self.abstract_space_mut().boundaries.push(boundary);
    }

    fn extend_boundaries(
        &mut self,
        boundaries: impl IntoIterator<Item = AbstractSpaceBoundaryProperty>,
    ) {
        self.abstract_space_mut().boundaries.extend(boundaries);
    }
}

impl AsAbstractSpace for AbstractSpace {
    fn abstract_space(&self) -> &AbstractSpace {
        self
    }
}

impl AsAbstractSpaceMut for AbstractSpace {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_city_object_traits!($type);

        impl $crate::model::core::AsAbstractCityObject for $type {
            fn abstract_city_object(&self) -> &$crate::model::core::AbstractCityObject {
                &<$type as $crate::model::core::AsAbstractSpace>::abstract_space(self)
                    .abstract_city_object
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_city_object_mut_traits!($type);

        impl $crate::model::core::AsAbstractCityObjectMut for $type {
            fn abstract_city_object_mut(&mut self) -> &mut $crate::model::core::AbstractCityObject {
                &mut <$type as $crate::model::core::AsAbstractSpaceMut>::abstract_space_mut(self)
                    .abstract_city_object
            }
        }
    };
}

impl_abstract_space_traits!(AbstractSpace);
impl_abstract_space_mut_traits!(AbstractSpace);

impl IterFeatures for AbstractSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_city_object.iter_features().chain(
                self.boundaries
                    .iter()
                    .filter_map(|x| x.object())
                    .flat_map(|x| x.iter_features()),
            ),
        )
    }
}

impl ForEachFeatureMut for AbstractSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_city_object.for_each_feature_mut(f);
        for prop in &mut self.boundaries {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for AbstractSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.lod1_solid
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod2_solid
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod3_solid
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod0_multi_surface
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod2_multi_surface
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod3_multi_surface
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod0_multi_curve
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod2_multi_curve
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod3_multi_curve
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl ApplyTransform for AbstractSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        if let Some(solid) = self.lod1_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_transform(m);
        }
        if let Some(solid) = self.lod2_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_transform(m);
        }
        if let Some(solid) = self.lod3_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_transform(m);
        }

        if let Some(multi_surface) = self
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_transform(m);
        }
        if let Some(multi_surface) = self
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_transform(m);
        }
        if let Some(multi_surface) = self
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_transform(m);
        }

        if let Some(multi_curve) = self.lod0_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_transform(m);
        }
        if let Some(multi_curve) = self.lod2_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_transform(m);
        }
        if let Some(multi_curve) = self.lod3_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_transform(m);
        }

        for prop in &mut self.boundaries {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(solid) = self.lod1_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_isometry(isometry);
        }
        if let Some(solid) = self.lod2_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_isometry(isometry);
        }
        if let Some(solid) = self.lod3_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_isometry(isometry);
        }

        if let Some(multi_surface) = self
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_isometry(isometry);
        }
        if let Some(multi_surface) = self
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_isometry(isometry);
        }
        if let Some(multi_surface) = self
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_isometry(isometry);
        }

        if let Some(multi_curve) = self.lod0_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_isometry(isometry);
        }
        if let Some(multi_curve) = self.lod2_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_isometry(isometry);
        }
        if let Some(multi_curve) = self.lod3_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_isometry(isometry);
        }

        for prop in &mut self.boundaries {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(solid) = self.lod1_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_translation(vector);
        }
        if let Some(solid) = self.lod2_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_translation(vector);
        }
        if let Some(solid) = self.lod3_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_translation(vector);
        }

        if let Some(multi_surface) = self
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_translation(vector);
        }
        if let Some(multi_surface) = self
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_translation(vector);
        }
        if let Some(multi_surface) = self
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_translation(vector);
        }

        if let Some(multi_curve) = self.lod0_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_translation(vector);
        }
        if let Some(multi_curve) = self.lod2_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_translation(vector);
        }
        if let Some(multi_curve) = self.lod3_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_translation(vector);
        }

        for prop in &mut self.boundaries {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(solid) = self.lod1_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_rotation(rotation);
        }
        if let Some(solid) = self.lod2_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_rotation(rotation);
        }
        if let Some(solid) = self.lod3_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_rotation(rotation);
        }

        if let Some(multi_surface) = self
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_rotation(rotation);
        }
        if let Some(multi_surface) = self
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_rotation(rotation);
        }
        if let Some(multi_surface) = self
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_rotation(rotation);
        }

        if let Some(multi_curve) = self.lod0_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_rotation(rotation);
        }
        if let Some(multi_curve) = self.lod2_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_rotation(rotation);
        }
        if let Some(multi_curve) = self.lod3_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_rotation(rotation);
        }

        for prop in &mut self.boundaries {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(solid) = self.lod1_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_scale(scale);
        }
        if let Some(solid) = self.lod2_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_scale(scale);
        }
        if let Some(solid) = self.lod3_solid.as_mut().and_then(|p| p.object_mut()) {
            solid.apply_scale(scale);
        }

        if let Some(multi_surface) = self
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_scale(scale);
        }
        if let Some(multi_surface) = self
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_scale(scale);
        }
        if let Some(multi_surface) = self
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_surface.apply_scale(scale);
        }

        if let Some(multi_curve) = self.lod0_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_scale(scale);
        }
        if let Some(multi_curve) = self.lod2_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_scale(scale);
        }
        if let Some(multi_curve) = self.lod3_multi_curve.as_mut().and_then(|p| p.object_mut()) {
            multi_curve.apply_scale(scale);
        }

        for prop in &mut self.boundaries {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
    }
}
