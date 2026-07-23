use crate::impl_abstract_city_object_mut_traits;
use crate::impl_abstract_city_object_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::abstract_space_kind::AbstractSpaceKind;
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{
    AbstractCityObject, AbstractSpaceBoundaryKind, AsAbstractCityObject, AsAbstractCityObjectMut,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractCityObjectKind {
    AbstractSpaceKind(AbstractSpaceKind),
    AbstractSpaceBoundaryKind(AbstractSpaceBoundaryKind),
}

impl AsAbstractCityObject for AbstractCityObjectKind {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::AbstractSpaceKind(x) => x.abstract_city_object(),
            Self::AbstractSpaceBoundaryKind(x) => x.abstract_city_object(),
        }
    }
}

impl AsAbstractCityObjectMut for AbstractCityObjectKind {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject {
        match self {
            Self::AbstractSpaceKind(x) => x.abstract_city_object_mut(),
            Self::AbstractSpaceBoundaryKind(x) => x.abstract_city_object_mut(),
        }
    }
}

impl_abstract_city_object_traits!(AbstractCityObjectKind);
impl_abstract_city_object_mut_traits!(AbstractCityObjectKind);

impl HasFeatureType for AbstractCityObjectKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractSpaceKind(x) => x.feature_type(),
            Self::AbstractSpaceBoundaryKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_city_object_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractCityObjectKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractCityObjectKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind!(AbstractCityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_city_object_kind!($variant, $variant);
    };
}
impl_from_for_city_object_kind!(AbstractSpaceKind);
impl_from_for_city_object_kind!(AbstractSpaceBoundaryKind);

#[macro_export]
macro_rules! impl_try_from_for_city_object_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractCityObjectKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractCityObjectKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractCityObjectKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind!(AbstractCityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_city_object_kind!($variant, $variant);
    };
}
impl_try_from_for_city_object_kind!(AbstractSpaceKind);
impl_try_from_for_city_object_kind!(AbstractSpaceBoundaryKind);

impl IterFeatures for AbstractCityObjectKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.iter_features(),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractCityObjectKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.for_each_feature_mut(f),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractCityObjectKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.compute_envelope(),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractCityObjectKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.apply_transform(m),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.apply_isometry(isometry),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.apply_translation(vector),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.apply_rotation(rotation),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractCityObjectKind::AbstractSpaceKind(x) => x.apply_scale(scale),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => x.apply_scale(scale),
        }
    }
}
