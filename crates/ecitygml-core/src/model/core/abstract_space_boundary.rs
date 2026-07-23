use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractCityObject, AsAbstractCityObject, AsAbstractCityObjectMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSpaceBoundary {
    pub(crate) abstract_city_object: AbstractCityObject,
}

impl AbstractSpaceBoundary {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_city_object(AbstractCityObject::new(id))
    }

    pub fn from_abstract_city_object(abstract_city_object: AbstractCityObject) -> Self {
        Self {
            abstract_city_object,
        }
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
                &<$type as $crate::model::core::AsAbstractSpaceBoundary>::abstract_space_boundary(
                    self,
                )
                .abstract_city_object
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_space_boundary_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_city_object_mut_traits!($type);

        impl $crate::model::core::AsAbstractCityObjectMut for $type {
            fn abstract_city_object_mut(&mut self) -> &mut $crate::model::core::AbstractCityObject {
                &mut <$type as $crate::model::core::AsAbstractSpaceBoundaryMut>::abstract_space_boundary_mut(self).abstract_city_object
            }
        }
    };
}

impl_abstract_space_boundary_traits!(AbstractSpaceBoundary);
impl_abstract_space_boundary_mut_traits!(AbstractSpaceBoundary);

impl IterFeatures for AbstractSpaceBoundary {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_city_object.iter_features()
    }
}

impl ForEachFeatureMut for AbstractSpaceBoundary {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_city_object.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractSpaceBoundary {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_city_object.compute_envelope()
    }
}

impl ApplyTransform for AbstractSpaceBoundary {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_city_object.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_city_object.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_city_object.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_city_object.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_city_object.apply_scale(scale);
    }
}
