use crate::impl_abstract_vegetation_object_mut_traits;
use crate::impl_abstract_vegetation_object_traits;

use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
    PlantCover, SolitaryVegetationObject,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractVegetationObjectKind {
    PlantCover(PlantCover),
    SolitaryVegetationObject(SolitaryVegetationObject),
}

impl AsAbstractVegetationObject for AbstractVegetationObjectKind {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.abstract_vegetation_object(),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
                x.abstract_vegetation_object()
            }
        }
    }
}

impl AsAbstractVegetationObjectMut for AbstractVegetationObjectKind {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.abstract_vegetation_object_mut(),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
                x.abstract_vegetation_object_mut()
            }
        }
    }
}

impl_abstract_vegetation_object_traits!(AbstractVegetationObjectKind);
impl_abstract_vegetation_object_mut_traits!(AbstractVegetationObjectKind);

impl HasFeatureType for AbstractVegetationObjectKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PlantCover(x) => x.feature_type(),
            Self::SolitaryVegetationObject(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_vegetation_object_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::vegetation::AbstractVegetationObjectKind {
            fn from(x: $type) -> Self {
                $crate::model::vegetation::AbstractVegetationObjectKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(AbstractVegetationObjectKind, $type);
    };
}
impl_from_vegetation_object_kind!(PlantCover);
impl_from_vegetation_object_kind!(SolitaryVegetationObject);

#[macro_export]
macro_rules! impl_try_from_vegetation_object_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::vegetation::AbstractVegetationObjectKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::vegetation::AbstractVegetationObjectKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::vegetation::AbstractVegetationObjectKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(AbstractVegetationObjectKind, $type);
    };
}
impl_try_from_vegetation_object_kind!(PlantCover);
impl_try_from_vegetation_object_kind!(SolitaryVegetationObject);

impl IterFeatures for AbstractVegetationObjectKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.iter_features(),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractVegetationObjectKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.for_each_feature_mut(f),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractVegetationObjectKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.compute_envelope(),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractVegetationObjectKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.apply_transform(m),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.apply_isometry(isometry),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.apply_translation(vector),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.apply_rotation(rotation),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractVegetationObjectKind::PlantCover(x) => x.apply_scale(scale),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => x.apply_scale(scale),
        }
    }
}
