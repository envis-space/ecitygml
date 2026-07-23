use crate::impl_abstract_space_boundary_mut_traits;
use crate::impl_abstract_space_boundary_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractSpaceBoundary, AbstractThematicSurfaceKind, AsAbstractSpaceBoundary,
    AsAbstractSpaceBoundaryMut,
};
use crate::model::relief::{AbstractReliefComponentKind, ReliefFeature};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractSpaceBoundaryKind {
    AbstractThematicSurfaceKind(AbstractThematicSurfaceKind),
    ReliefFeature(ReliefFeature),
    AbstractReliefComponentKind(AbstractReliefComponentKind),
}

impl AsAbstractSpaceBoundary for AbstractSpaceBoundaryKind {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => {
                x.abstract_space_boundary()
            }
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.abstract_space_boundary(),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => {
                x.abstract_space_boundary()
            }
        }
    }
}

impl AsAbstractSpaceBoundaryMut for AbstractSpaceBoundaryKind {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => {
                x.abstract_space_boundary_mut()
            }
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.abstract_space_boundary_mut(),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => {
                x.abstract_space_boundary_mut()
            }
        }
    }
}

impl_abstract_space_boundary_traits!(AbstractSpaceBoundaryKind);
impl_abstract_space_boundary_mut_traits!(AbstractSpaceBoundaryKind);

impl HasFeatureType for AbstractSpaceBoundaryKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractThematicSurfaceKind(x) => x.feature_type(),
            Self::ReliefFeature(x) => x.feature_type(),
            Self::AbstractReliefComponentKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_boundary_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractSpaceBoundaryKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractSpaceBoundaryKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind!(AbstractSpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_boundary_kind!($variant, $variant);
    };
}
impl_from_for_space_boundary_kind!(ReliefFeature);
impl_from_for_space_boundary_kind!(AbstractThematicSurfaceKind);
impl_from_for_space_boundary_kind!(AbstractReliefComponentKind);

#[macro_export]
macro_rules! impl_try_from_for_space_boundary_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractSpaceBoundaryKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractSpaceBoundaryKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractSpaceBoundaryKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind!(AbstractSpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_boundary_kind!($variant, $variant);
    };
}
impl_try_from_for_space_boundary_kind!(ReliefFeature);

impl IterFeatures for AbstractSpaceBoundaryKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.iter_features(),
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.iter_features(),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractSpaceBoundaryKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.for_each_feature_mut(f),
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.for_each_feature_mut(f),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractSpaceBoundaryKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.compute_envelope(),
            AbstractSpaceBoundaryKind::ReliefFeature(_) => None,
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractSpaceBoundaryKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.apply_transform(m),
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.apply_transform(m),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.apply_isometry(isometry),
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.apply_isometry(isometry),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => {
                x.apply_translation(vector)
            }
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.apply_translation(vector),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.apply_rotation(rotation),
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.apply_rotation(rotation),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => x.apply_scale(scale),
            AbstractSpaceBoundaryKind::ReliefFeature(x) => x.apply_scale(scale),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => x.apply_scale(scale),
        }
    }
}
