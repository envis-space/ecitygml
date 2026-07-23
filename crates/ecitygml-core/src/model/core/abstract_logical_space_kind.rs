use crate::impl_abstract_logical_space_mut_traits;
use crate::impl_abstract_logical_space_traits;
use crate::model::building::AbstractBuildingSubdivisionKind;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractLogicalSpaceKind {
    AbstractBuildingSubdivisionKind(AbstractBuildingSubdivisionKind),
}

impl AsAbstractLogicalSpace for AbstractLogicalSpaceKind {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                x.abstract_logical_space()
            }
        }
    }
}

impl AsAbstractLogicalSpaceMut for AbstractLogicalSpaceKind {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                x.abstract_logical_space_mut()
            }
        }
    }
}

impl_abstract_logical_space_traits!(AbstractLogicalSpaceKind);
impl_abstract_logical_space_mut_traits!(AbstractLogicalSpaceKind);

impl HasFeatureType for AbstractLogicalSpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractBuildingSubdivisionKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_logical_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractLogicalSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractLogicalSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind!(AbstractLogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_logical_space_kind!($variant, $variant);
    };
}
impl_from_for_logical_space_kind!(AbstractBuildingSubdivisionKind);

#[macro_export]
macro_rules! impl_try_from_for_logical_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractLogicalSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractLogicalSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractLogicalSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind!(AbstractLogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_logical_space_kind!($variant, $variant);
    };
}

impl IterFeatures for AbstractLogicalSpaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractLogicalSpaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                x.for_each_feature_mut(f)
            }
        }
    }
}

impl ComputeEnvelope for AbstractLogicalSpaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractLogicalSpaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                x.apply_isometry(isometry)
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                x.apply_rotation(rotation)
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => x.apply_scale(scale),
        }
    }
}
