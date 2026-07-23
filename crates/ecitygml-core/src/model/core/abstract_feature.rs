use egml::model::base::{AsAbstractGml, AsAbstractGmlMut, Id};
use egml::model::common::ApplyTransform;
use egml::model::feature::{
    AsAbstractFeature as GmlAsAbstractFeature, AsAbstractFeatureMut as GmlAsAbstractFeatureMut,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFeature {
    pub(crate) abstract_feature: egml::model::feature::AbstractFeature,
}

impl AbstractFeature {
    pub fn new(id: Id) -> Self {
        let mut abstract_feature = egml::model::feature::AbstractFeature::new();
        abstract_feature.set_id(id);

        Self { abstract_feature }
    }

    pub fn from_abstract_feature(abstract_feature: egml::model::feature::AbstractFeature) -> Self {
        Self { abstract_feature }
    }
}

pub trait AsAbstractFeature {
    fn abstract_feature(&self) -> &AbstractFeature;

    fn feature_id(&self) -> &Id {
        <Self as AsAbstractFeature>::abstract_feature(self)
            .abstract_gml()
            .id()
            .as_ref()
            .expect("id must be set for AbstractFeature")
    }
}

pub trait AsAbstractFeatureMut: AsAbstractFeature {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature;
}

impl AsAbstractFeature for AbstractFeature {
    fn abstract_feature(&self) -> &AbstractFeature {
        self
    }
}

impl AsAbstractFeatureMut for AbstractFeature {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_feature_traits {
    ($type:ty) => {
        egml::impl_abstract_feature_traits!($type);

        impl egml::model::feature::AsAbstractFeature for $type {
            fn abstract_feature(&self) -> &egml::model::feature::AbstractFeature {
                &<$type as $crate::model::core::AsAbstractFeature>::abstract_feature(self)
                    .abstract_feature
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_feature_mut_traits {
    ($type:ty) => {
        egml::impl_abstract_feature_mut_traits!($type);

        impl egml::model::feature::AsAbstractFeatureMut for $type {
            fn abstract_feature_mut(&mut self) -> &mut egml::model::feature::AbstractFeature {
                &mut <$type as $crate::model::core::AsAbstractFeatureMut>::abstract_feature_mut(
                    self,
                )
                .abstract_feature
            }
        }
    };
}

crate::impl_abstract_feature_traits!(AbstractFeature);
crate::impl_abstract_feature_mut_traits!(AbstractFeature);

impl ApplyTransform for AbstractFeature {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        if let Some(bounding_shape) = self.abstract_feature.bounded_by_mut() {
            bounding_shape.apply_transform(m);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(bounding_shape) = self.abstract_feature.bounded_by_mut() {
            bounding_shape.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(bounding_shape) = self.abstract_feature.bounded_by_mut() {
            bounding_shape.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(bounding_shape) = self.abstract_feature.bounded_by_mut() {
            bounding_shape.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(bounding_shape) = self.abstract_feature.bounded_by_mut() {
            bounding_shape.apply_scale(scale);
        }
    }
}
