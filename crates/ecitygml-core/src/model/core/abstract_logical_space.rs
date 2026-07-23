use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractLogicalSpace {
    pub(crate) abstract_space: AbstractSpace,
}

impl AbstractLogicalSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_space(AbstractSpace::new(id))
    }

    pub fn from_abstract_space(abstract_space: AbstractSpace) -> Self {
        Self { abstract_space }
    }
}

pub trait AsAbstractLogicalSpace: AsAbstractSpace {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace;
}

pub trait AsAbstractLogicalSpaceMut: AsAbstractSpaceMut + AsAbstractLogicalSpace {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace;
}

impl AsAbstractLogicalSpace for AbstractLogicalSpace {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        self
    }
}

impl AsAbstractLogicalSpaceMut for AbstractLogicalSpace {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_logical_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_traits!($type);

        impl $crate::model::core::AsAbstractSpace for $type {
            fn abstract_space(&self) -> &$crate::model::core::AbstractSpace {
                &<$type as $crate::model::core::AsAbstractLogicalSpace>::abstract_logical_space(
                    self,
                )
                .abstract_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_logical_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractSpaceMut for $type {
            fn abstract_space_mut(&mut self) -> &mut $crate::model::core::AbstractSpace {
                &mut <$type as $crate::model::core::AsAbstractLogicalSpaceMut>::abstract_logical_space_mut(self).abstract_space
            }
        }
    };
}

impl_abstract_logical_space_traits!(AbstractLogicalSpace);
impl_abstract_logical_space_mut_traits!(AbstractLogicalSpace);

impl IterFeatures for AbstractLogicalSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_space.iter_features()
    }
}

impl ForEachFeatureMut for AbstractLogicalSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractLogicalSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_space.compute_envelope()
    }
}

impl ApplyTransform for AbstractLogicalSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_space.apply_scale(scale);
    }
}
