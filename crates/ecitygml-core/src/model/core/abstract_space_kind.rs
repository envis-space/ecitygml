use crate::impl_abstract_space_mut_traits;
use crate::impl_abstract_space_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::abstract_logical_space_kind::AbstractLogicalSpaceKind;
use crate::model::core::abstract_physical_space_kind::AbstractPhysicalSpaceKind;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractSpaceKind {
    AbstractLogicalSpaceKind(AbstractLogicalSpaceKind),
    AbstractPhysicalSpaceKind(AbstractPhysicalSpaceKind),
}

impl AsAbstractSpace for AbstractSpaceKind {
    fn abstract_space(&self) -> &AbstractSpace {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.abstract_space(),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.abstract_space(),
        }
    }
}

impl AsAbstractSpaceMut for AbstractSpaceKind {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.abstract_space_mut(),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.abstract_space_mut(),
        }
    }
}

impl_abstract_space_traits!(AbstractSpaceKind);
impl_abstract_space_mut_traits!(AbstractSpaceKind);

impl HasFeatureType for AbstractSpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractLogicalSpaceKind(x) => x.feature_type(),
            Self::AbstractPhysicalSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind!(AbstractSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_kind!($variant, $variant);
    };
}
impl_from_for_space_kind!(AbstractLogicalSpaceKind);
impl_from_for_space_kind!(AbstractPhysicalSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind!(AbstractSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_kind!($variant, $variant);
    };
}
impl_try_from_for_space_kind!(AbstractLogicalSpaceKind);
impl_try_from_for_space_kind!(AbstractPhysicalSpaceKind);

impl IterFeatures for AbstractSpaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.iter_features(),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractSpaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.for_each_feature_mut(f),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractSpaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.compute_envelope(),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractSpaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.apply_transform(m),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.apply_isometry(isometry),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.apply_translation(vector),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.apply_rotation(rotation),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => x.apply_scale(scale),
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => x.apply_scale(scale),
        }
    }
}
