use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractLogicalSpace {
    pub(crate) abstract_space: AbstractSpace,
}

impl AbstractLogicalSpace {
    pub fn new(abstract_space: AbstractSpace) -> Self {
        Self { abstract_space }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_space.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_space.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_space.apply_transform(m);
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
                use $crate::model::core::AsAbstractLogicalSpace;
                &self.abstract_logical_space().abstract_space
            }
        }

        impl $crate::model::core::AsAbstractSpaceMut for $type {
            fn abstract_space_mut(&mut self) -> &mut $crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractLogicalSpaceMut;
                &mut self.abstract_logical_space_mut().abstract_space
            }
        }
    };
}

impl_abstract_logical_space_traits!(AbstractLogicalSpace);
