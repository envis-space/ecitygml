use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSurfaceData {
    pub(crate) abstract_feature: AbstractFeature,
    is_front: Option<bool>,
}

impl AbstractSurfaceData {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_feature(AbstractFeature::new(id))
    }

    pub fn from_abstract_feature(abstract_feature: AbstractFeature) -> Self {
        Self {
            abstract_feature,
            is_front: None,
        }
    }
}
impl AbstractSurfaceData {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::empty()
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}
    pub fn compute_envelope(&self) -> Option<Envelope> {
        None
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_feature.apply_transform(m);
    }
}

pub trait AsAbstractSurfaceData: AsAbstractFeature {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData;

    fn is_front(&self) -> Option<&bool> {
        self.abstract_surface_data().is_front.as_ref()
    }
}

pub trait AsAbstractSurfaceDataMut: AsAbstractFeatureMut + AsAbstractSurfaceData {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData;

    fn set_is_front(&mut self, is_front: Option<bool>) {
        self.abstract_surface_data_mut().is_front = is_front;
    }
}

impl AsAbstractSurfaceData for AbstractSurfaceData {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        self
    }
}

impl AsAbstractSurfaceDataMut for AbstractSurfaceData {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_surface_data_traits {
    ($type:ty) => {
        impl $crate::model::core::AsAbstractFeature for $type {
            fn abstract_feature(&self) -> &$crate::model::core::AbstractFeature {
                use $crate::model::appearance::AsAbstractSurfaceData;
                &self.abstract_surface_data().abstract_feature
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_surface_data_mut_traits {
    ($type:ty) => {
        impl $crate::model::core::AsAbstractFeatureMut for $type {
            fn abstract_feature_mut(&mut self) -> &mut $crate::model::core::AbstractFeature {
                use $crate::model::appearance::AsAbstractSurfaceDataMut;
                &mut self.abstract_surface_data_mut().abstract_feature
            }
        }
    };
}

impl_abstract_surface_data_traits!(AbstractSurfaceData);
impl_abstract_surface_data_mut_traits!(AbstractSurfaceData);
