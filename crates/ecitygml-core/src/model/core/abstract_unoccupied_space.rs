use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractUnoccupiedSpace {
    pub(crate) abstract_physical_space: AbstractPhysicalSpace,
}

impl AbstractUnoccupiedSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_physical_space(AbstractPhysicalSpace::new(id))
    }

    pub fn from_abstract_physical_space(abstract_physical_space: AbstractPhysicalSpace) -> Self {
        Self {
            abstract_physical_space,
        }
    }
}
impl AbstractUnoccupiedSpace {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_physical_space.iter_features()
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_physical_space.for_each_feature_mut(f);
    }
    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_physical_space.compute_envelope()
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_physical_space.apply_transform(m);
    }
}

pub trait AsAbstractUnoccupiedSpace: AsAbstractPhysicalSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace;
}

pub trait AsAbstractUnoccupiedSpaceMut:
    AsAbstractPhysicalSpaceMut + AsAbstractUnoccupiedSpace
{
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace;
}

impl AsAbstractUnoccupiedSpace for AbstractUnoccupiedSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        self
    }
}

impl AsAbstractUnoccupiedSpaceMut for AbstractUnoccupiedSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_unoccupied_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_physical_space_traits!($type);

        impl $crate::model::core::AsAbstractPhysicalSpace for $type {
            fn abstract_physical_space(&self) -> &$crate::model::core::AbstractPhysicalSpace {
                use $crate::model::core::AsAbstractUnoccupiedSpace;
                &self.abstract_unoccupied_space().abstract_physical_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_unoccupied_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_physical_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractPhysicalSpaceMut for $type {
            fn abstract_physical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractPhysicalSpace {
                use $crate::model::core::AsAbstractUnoccupiedSpaceMut;
                &mut self.abstract_unoccupied_space_mut().abstract_physical_space
            }
        }
    };
}

impl_abstract_unoccupied_space_traits!(AbstractUnoccupiedSpace);
impl_abstract_unoccupied_space_mut_traits!(AbstractUnoccupiedSpace);
