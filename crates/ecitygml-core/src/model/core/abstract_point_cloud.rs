use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractPointCloud {
    pub(crate) abstract_feature: AbstractFeature,
}

impl AbstractPointCloud {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_feature(AbstractFeature::new(id))
    }

    pub fn from_abstract_feature(abstract_feature: AbstractFeature) -> Self {
        Self { abstract_feature }
    }
}

impl AbstractPointCloud {
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

pub trait AsAbstractPointCloud: AsAbstractFeature {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud;
}

pub trait AsAbstractPointCloudMut: AsAbstractFeatureMut + AsAbstractPointCloud {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud;
}

impl AsAbstractPointCloud for AbstractPointCloud {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        self
    }
}

impl AsAbstractPointCloudMut for AbstractPointCloud {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_point_cloud_traits {
    ($type:ty) => {
        impl $crate::model::core::AsAbstractFeature for $type {
            fn abstract_feature(&self) -> &$crate::model::core::AbstractFeature {
                use $crate::model::core::AsAbstractPointCloud;
                &self.abstract_point_cloud().abstract_feature
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_point_cloud_mut_traits {
    ($type:ty) => {
        impl $crate::model::core::AsAbstractFeatureMut for $type {
            fn abstract_feature_mut(&mut self) -> &mut $crate::model::core::AbstractFeature {
                use $crate::model::core::AsAbstractPointCloudMut;
                &mut self.abstract_point_cloud_mut().abstract_feature
            }
        }
    };
}

impl_abstract_point_cloud_traits!(AbstractPointCloud);
impl_abstract_point_cloud_mut_traits!(AbstractPointCloud);
