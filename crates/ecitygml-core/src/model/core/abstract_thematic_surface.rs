use crate::model::common::LevelOfDetail;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut, PointCloudProperty,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::{MultiCurveProperty, MultiSurfaceProperty};
use nalgebra::Isometry3;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractThematicSurface {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    lod0_multi_curve: Option<MultiCurveProperty>,
    lod0_multi_surface: Option<MultiSurfaceProperty>,
    lod1_multi_surface: Option<MultiSurfaceProperty>,
    lod2_multi_surface: Option<MultiSurfaceProperty>,
    lod3_multi_surface: Option<MultiSurfaceProperty>,
    point_cloud: Option<PointCloudProperty>,
}

impl AbstractThematicSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_space_boundary(AbstractSpaceBoundary::new(id))
    }

    pub fn from_abstract_space_boundary(abstract_space_boundary: AbstractSpaceBoundary) -> Self {
        Self {
            abstract_space_boundary,
            lod0_multi_curve: None,
            lod0_multi_surface: None,
            lod1_multi_surface: None,
            lod2_multi_surface: None,
            lod3_multi_surface: None,
            point_cloud: None,
        }
    }
}
impl AbstractThematicSurface {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_space_boundary.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space_boundary.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_space_boundary.compute_envelope(),
            self.lod0_multi_surface
                .as_ref()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
            self.lod1_multi_surface
                .as_ref()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
            self.lod2_multi_surface
                .as_ref()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
            self.lod3_multi_surface
                .as_ref()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
            self.lod0_multi_curve
                .as_ref()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

pub trait AsAbstractThematicSurface: AsAbstractSpaceBoundary {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface;

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

    fn point_cloud(&self) -> Option<&PointCloudProperty> {
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

    fn set_point_cloud(&mut self, value: Option<PointCloudProperty>) {
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

    fn point_cloud_mut(&mut self) -> Option<&mut PointCloudProperty> {
        self.abstract_thematic_surface_mut().point_cloud.as_mut()
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(multi_curve) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_curve
            .as_mut()
            .and_then(|p| p.object.as_mut())
        {
            multi_curve.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod0_multi_surface
            .as_mut()
            .and_then(|p| p.object.as_mut())
        {
            surface.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod1_multi_surface
            .as_mut()
            .and_then(|p| p.object.as_mut())
        {
            surface.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod2_multi_surface
            .as_mut()
            .and_then(|p| p.object.as_mut())
        {
            surface.apply_transform(m);
        }
        if let Some(surface) = self
            .abstract_thematic_surface_mut()
            .lod3_multi_surface
            .as_mut()
            .and_then(|p| p.object.as_mut())
        {
            surface.apply_transform(m);
        }
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
                use $crate::model::core::AsAbstractThematicSurface;
                &self.abstract_thematic_surface().abstract_space_boundary
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
                use $crate::model::core::AsAbstractThematicSurfaceMut;
                &mut self.abstract_thematic_surface_mut().abstract_space_boundary
            }
        }
    };
}

impl_abstract_thematic_surface_traits!(AbstractThematicSurface);
impl_abstract_thematic_surface_mut_traits!(AbstractThematicSurface);
