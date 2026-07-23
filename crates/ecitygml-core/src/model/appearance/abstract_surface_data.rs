use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

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
        $crate::impl_abstract_feature_traits!($type);

        impl $crate::model::core::AsAbstractFeature for $type {
            fn abstract_feature(&self) -> &$crate::model::core::AbstractFeature {
                &<$type as $crate::model::appearance::AsAbstractSurfaceData>::abstract_surface_data(
                    self,
                )
                .abstract_feature
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_surface_data_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_feature_mut_traits!($type);

        impl $crate::model::core::AsAbstractFeatureMut for $type {
            fn abstract_feature_mut(&mut self) -> &mut $crate::model::core::AbstractFeature {
                &mut <$type as $crate::model::appearance::AsAbstractSurfaceDataMut>::abstract_surface_data_mut(self).abstract_feature
            }
        }
    };
}

impl_abstract_surface_data_traits!(AbstractSurfaceData);
impl_abstract_surface_data_mut_traits!(AbstractSurfaceData);

impl IterFeatures for AbstractSurfaceData {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::empty())
    }
}

impl ForEachFeatureMut for AbstractSurfaceData {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}
}

impl ComputeEnvelope for AbstractSurfaceData {
    fn compute_envelope(&self) -> Option<Envelope> {
        None
    }
}

impl ApplyTransform for AbstractSurfaceData {
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
