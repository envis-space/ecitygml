use crate::impl_abstract_space_boundary_mut_traits;
use crate::impl_abstract_space_boundary_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
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
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => x.iter_features(),
            SpaceBoundaryKind::ReliefFeature(x) => x.iter_features(),
            SpaceBoundaryKind::ReliefComponentKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
impl_abstract_space_boundary_mut_traits!(SpaceBoundaryKind);

impl HasFeatureType for SpaceBoundaryKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::ThematicSurfaceKind(x) => x.feature_type(),
            Self::ReliefFeature(x) => x.feature_type(),
            Self::ReliefComponentKind(x) => x.feature_type(),
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

#[macro_export]
macro_rules! impl_try_from_for_space_boundary_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::SpaceBoundaryKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::SpaceBoundaryKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::SpaceBoundaryKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind!(SpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_boundary_kind!($variant, $variant);
    };
}
impl_try_from_for_space_boundary_kind!(ReliefFeature);
