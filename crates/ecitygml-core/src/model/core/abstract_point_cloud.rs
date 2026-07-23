use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

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
        $crate::impl_abstract_feature_traits!($type);

        impl $crate::model::core::AsAbstractFeature for $type {
            fn abstract_feature(&self) -> &$crate::model::core::AbstractFeature {
                &<$type as $crate::model::core::AsAbstractPointCloud>::abstract_point_cloud(self)
                    .abstract_feature
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_point_cloud_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_feature_mut_traits!($type);

        impl $crate::model::core::AsAbstractFeatureMut for $type {
            fn abstract_feature_mut(&mut self) -> &mut $crate::model::core::AbstractFeature {
                &mut <$type as $crate::model::core::AsAbstractPointCloudMut>::abstract_point_cloud_mut(self).abstract_feature
            }
        }
    };
}

impl_abstract_point_cloud_traits!(AbstractPointCloud);
impl_abstract_point_cloud_mut_traits!(AbstractPointCloud);

impl IterFeatures for AbstractPointCloud {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::empty())
    }
}

impl ForEachFeatureMut for AbstractPointCloud {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}
}

impl ComputeEnvelope for AbstractPointCloud {
    fn compute_envelope(&self) -> Option<Envelope> {
        None
    }
}

impl ApplyTransform for AbstractPointCloud {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_feature.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_feature.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_feature.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_feature.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_feature.apply_scale(scale);
    }
}
