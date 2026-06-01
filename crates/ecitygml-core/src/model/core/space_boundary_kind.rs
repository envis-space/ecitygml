use crate::impl_abstract_space_boundary_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut, ThematicSurfaceKind,
};
use crate::model::relief::{ReliefComponentKind, ReliefFeature};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum SpaceBoundaryKind {
    ThematicSurfaceKind(ThematicSurfaceKind),
    ReliefFeature(ReliefFeature),
    ReliefComponentKind(ReliefComponentKind),
}

impl SpaceBoundaryKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.iter_features(),
            SpaceBoundaryKind::ReliefFeature(x) => x.iter_features(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.for_each_feature_mut(f),
            SpaceBoundaryKind::ReliefFeature(x) => x.for_each_feature_mut(f),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.compute_envelope(),
            SpaceBoundaryKind::ReliefFeature(_) => None,
            SpaceBoundaryKind::ReliefComponentKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.recompute_bounding_shape(),
            SpaceBoundaryKind::ReliefFeature(x) => x.recompute_bounding_shape(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.apply_transform(m),
            SpaceBoundaryKind::ReliefFeature(x) => x.apply_transform(m),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractSpaceBoundary for SpaceBoundaryKind {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.abstract_space_boundary(),
            SpaceBoundaryKind::ReliefFeature(x) => x.abstract_space_boundary(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.abstract_space_boundary(),
        }
    }
}

impl AsAbstractSpaceBoundaryMut for SpaceBoundaryKind {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.abstract_space_boundary_mut(),
            SpaceBoundaryKind::ReliefFeature(x) => x.abstract_space_boundary_mut(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.abstract_space_boundary_mut(),
        }
    }
}

impl_abstract_space_boundary_traits!(SpaceBoundaryKind);

impl<'a> From<&'a SpaceBoundaryKind> for FeatureRef<'a> {
    fn from(item: &'a SpaceBoundaryKind) -> Self {
        match item {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.into(),
            SpaceBoundaryKind::ReliefFeature(x) => x.into(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a SpaceBoundaryKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a SpaceBoundaryKind) -> Result<Self, ()> {
        match item {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.try_into(),
            SpaceBoundaryKind::ReliefFeature(x) => Ok(x.into()),
            SpaceBoundaryKind::ReliefComponentKind(_) => Err(()),
        }
    }
}

impl<'a> From<&'a mut SpaceBoundaryKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut SpaceBoundaryKind) -> Self {
        match item {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.into(),
            SpaceBoundaryKind::ReliefFeature(x) => x.into(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_boundary_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::SpaceBoundaryKind {
            fn from(x: $type) -> Self {
                $crate::model::core::SpaceBoundaryKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind!(SpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_boundary_kind!($variant, $variant);
    };
}
impl_from_for_space_boundary_kind!(ReliefFeature);
impl_from_for_space_boundary_kind!(ThematicSurfaceKind);
impl_from_for_space_boundary_kind!(ReliefComponentKind);
