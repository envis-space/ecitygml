use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFillingElement {
    pub abstract_occupied_space: AbstractOccupiedSpace,
}

impl AbstractFillingElement {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_occupied_space.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_occupied_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_occupied_space.apply_transform(m);
    }
}

pub trait AsAbstractFillingElement: AsAbstractOccupiedSpace {
    fn abstract_filling_element(&self) -> &AbstractFillingElement;
}

pub trait AsAbstractFillingElementMut:
    AsAbstractOccupiedSpaceMut + AsAbstractFillingElement
{
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement;
}

impl AsAbstractFillingElement for AbstractFillingElement {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        self
    }
}

impl AsAbstractFillingElementMut for AbstractFillingElement {
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_filling_element_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractFillingElement;
                &self.abstract_filling_element().abstract_occupied_space
            }
        }

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractFillingElementMut;
                &mut self.abstract_filling_element_mut().abstract_occupied_space
            }
        }
    };
}

impl_abstract_filling_element_traits!(AbstractFillingElement);
