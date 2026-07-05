use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

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

impl AbstractLogicalSpace {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_space.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
    };
}

#[macro_export]
macro_rules! impl_abstract_logical_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractSpaceMut for $type {
            fn abstract_space_mut(&mut self) -> &mut $crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractLogicalSpaceMut;
                &mut self.abstract_logical_space_mut().abstract_space
            }
        }
    };
}

impl_abstract_logical_space_traits!(AbstractLogicalSpace);
impl_abstract_logical_space_mut_traits!(AbstractLogicalSpace);
