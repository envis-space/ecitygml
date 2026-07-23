use crate::impl_abstract_physical_space_mut_traits;
use crate::impl_abstract_physical_space_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::abstract_occupied_space_kind::AbstractOccupiedSpaceKind;
use crate::model::core::abstract_unoccupied_space_kind::AbstractUnoccupiedSpaceKind;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractPhysicalSpaceKind {
    AbstractOccupiedSpaceKind(AbstractOccupiedSpaceKind),
    AbstractUnoccupiedSpaceKind(AbstractUnoccupiedSpaceKind),
}

impl AsAbstractPhysicalSpace for AbstractPhysicalSpaceKind {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.abstract_physical_space(),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => {
                x.abstract_physical_space()
            }
        }
    }
}

impl AsAbstractPhysicalSpaceMut for AbstractPhysicalSpaceKind {
    fn abstract_physical_space_mut(&mut self) -> &mut AbstractPhysicalSpace {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => {
                x.abstract_physical_space_mut()
            }
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => {
                x.abstract_physical_space_mut()
            }
        }
    }
}

impl_abstract_physical_space_traits!(AbstractPhysicalSpaceKind);
impl_abstract_physical_space_mut_traits!(AbstractPhysicalSpaceKind);

impl HasFeatureType for AbstractPhysicalSpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractOccupiedSpaceKind(x) => x.feature_type(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_physical_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractPhysicalSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractPhysicalSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind!(AbstractPhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_physical_space_kind!($variant, $variant);
    };
}
impl_from_for_physical_space_kind!(AbstractOccupiedSpaceKind);
impl_from_for_physical_space_kind!(AbstractUnoccupiedSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_physical_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractPhysicalSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractPhysicalSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractPhysicalSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind!(AbstractPhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_physical_space_kind!($variant, $variant);
    };
}
impl_try_from_for_physical_space_kind!(AbstractOccupiedSpaceKind);
impl_try_from_for_physical_space_kind!(AbstractUnoccupiedSpaceKind);

impl IterFeatures for AbstractPhysicalSpaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.iter_features(),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractPhysicalSpaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.for_each_feature_mut(f),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractPhysicalSpaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.compute_envelope(),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractPhysicalSpaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.apply_transform(m),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.apply_isometry(isometry),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.apply_translation(vector),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.apply_rotation(rotation),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => x.apply_scale(scale),
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => x.apply_scale(scale),
        }
    }
}
