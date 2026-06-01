use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{AbstractCityObject, AsAbstractCityObject, AsAbstractCityObjectMut};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSpaceBoundary {
    pub(crate) abstract_city_object: AbstractCityObject,
}

impl AbstractSpaceBoundary {
    pub fn new(abstract_city_object: AbstractCityObject) -> Self {
        Self {
            abstract_city_object,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_city_object.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_city_object.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_city_object.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_city_object.apply_transform(m);
    }
}

pub trait AsAbstractSpaceBoundary: AsAbstractCityObject {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary;
}

pub trait AsAbstractSpaceBoundaryMut: AsAbstractCityObjectMut + AsAbstractSpaceBoundary {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary;
}

impl AsAbstractSpaceBoundary for AbstractSpaceBoundary {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        self
    }
}

impl AsAbstractSpaceBoundaryMut for AbstractSpaceBoundary {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_space_boundary_traits {
    ($type:ty) => {
        $crate::impl_abstract_city_object_traits!($type);

        impl $crate::model::core::AsAbstractCityObject for $type {
            fn abstract_city_object(&self) -> &$crate::model::core::AbstractCityObject {
                use $crate::model::core::AsAbstractSpaceBoundary;
                &self.abstract_space_boundary().abstract_city_object
            }
        }

        impl $crate::model::core::AsAbstractCityObjectMut for $type {
            fn abstract_city_object_mut(&mut self) -> &mut $crate::model::core::AbstractCityObject {
                use $crate::model::core::AsAbstractSpaceBoundaryMut;
                &mut self.abstract_space_boundary_mut().abstract_city_object
            }
        }
    };
}

impl_abstract_space_boundary_traits!(AbstractSpaceBoundary);
