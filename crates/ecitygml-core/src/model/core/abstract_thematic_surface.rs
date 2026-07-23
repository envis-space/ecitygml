use crate::model::common::{ForEachFeatureMut, IterFeatures, LevelOfDetail};
use crate::model::core::qualified_area::QualifiedArea;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractPointCloudProperty, AbstractSpaceBoundary, AsAbstractSpaceBoundary,
    AsAbstractSpaceBoundaryMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::{MultiCurveProperty, MultiSurfaceProperty};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractThematicSurface {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    areas: Vec<QualifiedArea>,
    lod0_multi_curve: Option<MultiCurveProperty>,
    lod0_multi_surface: Option<MultiSurfaceProperty>,
    lod1_multi_surface: Option<MultiSurfaceProperty>,
    lod2_multi_surface: Option<MultiSurfaceProperty>,
    lod3_multi_surface: Option<MultiSurfaceProperty>,
    point_cloud: Option<AbstractPointCloudProperty>,
}

impl AbstractThematicSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_space_boundary(AbstractSpaceBoundary::new(id))
    }

    pub fn from_abstract_space_boundary(abstract_space_boundary: AbstractSpaceBoundary) -> Self {
        Self {
            abstract_space_boundary,
            areas: Vec::new(),
            lod0_multi_curve: None,
            lod0_multi_surface: None,
            lod1_multi_surface: None,
            lod2_multi_surface: None,
            lod3_multi_surface: None,
            point_cloud: None,
        }
    }
}

pub trait AsAbstractThematicSurface: AsAbstractSpaceBoundary {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface;

    fn areas(&self) -> &[QualifiedArea] {
        self.abstract_thematic_surface().areas.as_ref()
    }

    fn lod0_multi_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_thematic_surface().lod0_multi_curve.as_ref()
    }

    fn lod0_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_thematic_surface().lod0_multi_surface.as_ref()
    }

    fn lod1_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_thematic_surface().lod1_multi_surface.as_ref()
    }

    fn lod2_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_thematic_surface().lod2_multi_surface.as_ref()
    }

    fn lod3_multi_surface(&self) -> Option<&MultiSurfaceProperty> {
        self.abstract_thematic_surface().lod3_multi_surface.as_ref()
    }

    fn point_cloud(&self) -> Option<&AbstractPointCloudProperty> {
        self.abstract_thematic_surface().point_cloud.as_ref()
    }

    fn multi_surfaces_by_lod(&self) -> HashMap<LevelOfDetail, &MultiSurfaceProperty> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod0_multi_surface() {
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod1_multi_surface() {
            map.insert(LevelOfDetail::One, x);
        }
        if let Some(x) = self.lod2_multi_surface() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_multi_surface() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }
}

pub trait AsAbstractThematicSurfaceMut:
    AsAbstractSpaceBoundaryMut + AsAbstractThematicSurface
{
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface;

    fn areas_mut(&mut self) -> &mut Vec<QualifiedArea> {
        &mut self.abstract_thematic_surface_mut().areas
    }

    fn set_areas(&mut self, values: Vec<QualifiedArea>) {
        self.abstract_thematic_surface_mut().areas = values;
    }

    fn push_area(&mut self, area: QualifiedArea) {
        self.abstract_thematic_surface_mut().areas.push(area);
    }

    fn extend_areas(&mut self, areas: impl IntoIterator<Item = QualifiedArea>) {
        self.abstract_thematic_surface_mut().areas.extend(areas);
    }

    fn set_lod0_multi_curve(&mut self, value: Option<MultiCurveProperty>) {
        self.abstract_thematic_surface_mut().lod0_multi_curve = value;
    }

    fn set_lod0_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_thematic_surface_mut().lod0_multi_surface = value;
    }

    fn set_lod1_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_thematic_surface_mut().lod1_multi_surface = value;
    }

    fn set_lod2_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_thematic_surface_mut().lod2_multi_surface = value;
    }

    fn set_lod3_multi_surface(&mut self, value: Option<MultiSurfaceProperty>) {
        self.abstract_thematic_surface_mut().lod3_multi_surface = value;
    }

    fn set_point_cloud(&mut self, value: Option<AbstractPointCloudProperty>) {
        self.abstract_thematic_surface_mut().point_cloud = value;
    }

    fn lod0_multi_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
    }

    fn lod0_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
    }

    fn lod1_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
    }

    fn lod2_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
    }

    fn lod3_multi_surface_mut(&mut self) -> Option<&mut MultiSurfaceProperty> {
        self.abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
    }

    fn point_cloud_mut(&mut self) -> Option<&mut AbstractPointCloudProperty> {
        self.abstract_thematic_surface_mut().point_cloud.as_mut()
    }
}

impl AsAbstractThematicSurface for AbstractThematicSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        self
    }
}

impl AsAbstractThematicSurfaceMut for AbstractThematicSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_thematic_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_boundary_traits!($type);

        impl $crate::model::core::AsAbstractSpaceBoundary for $type {
            fn abstract_space_boundary(&self) -> &$crate::model::core::AbstractSpaceBoundary {
                &<$type as $crate::model::core::AsAbstractThematicSurface>::abstract_thematic_surface(self).abstract_space_boundary
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_thematic_surface_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_boundary_mut_traits!($type);

        impl $crate::model::core::AsAbstractSpaceBoundaryMut for $type {
            fn abstract_space_boundary_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractSpaceBoundary {
                &mut <$type as $crate::model::core::AsAbstractThematicSurfaceMut>::abstract_thematic_surface_mut(self).abstract_space_boundary
            }
        }
    };
}

impl_abstract_thematic_surface_traits!(AbstractThematicSurface);
impl_abstract_thematic_surface_mut_traits!(AbstractThematicSurface);

impl IterFeatures for AbstractThematicSurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_space_boundary.iter_features(), /* TODO .chain(
                                                              self.point_cloud
                                                                  .iter()
                                                                  .filter_map(|x| x.object.as_ref())
                                                                  .flat_map(|x| x.iter_features()),
                                                          )*/
        )
    }
}

impl ForEachFeatureMut for AbstractThematicSurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space_boundary.for_each_feature_mut(f);
        if let Some(prop) = &mut self.point_cloud
            && let Some(x) = prop.object_mut()
        {
            x.for_each_feature_mut(f);
        }
    }
}

impl ComputeEnvelope for AbstractThematicSurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_space_boundary.compute_envelope(),
            self.lod0_multi_surface
                .as_ref()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod1_multi_surface
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
            self.point_cloud()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl ApplyTransform for AbstractThematicSurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        if let Some(multi_curve) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_curve.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_transform(m);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(multi_curve) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_curve.apply_isometry(isometry);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_isometry(isometry);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_isometry(isometry);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_isometry(isometry);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(multi_curve) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_curve.apply_translation(vector);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_translation(vector);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_translation(vector);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_translation(vector);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(multi_curve) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_curve.apply_rotation(rotation);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_rotation(rotation);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_rotation(rotation);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_rotation(rotation);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(multi_curve) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            multi_curve.apply_scale(scale);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_scale(scale);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_scale(scale);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_scale(scale);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            surface.apply_scale(scale);
        }
    }
}
