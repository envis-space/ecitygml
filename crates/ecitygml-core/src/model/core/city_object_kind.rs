use crate::impl_abstract_city_object_mut_traits;
use crate::impl_abstract_city_object_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::space_kind::SpaceKind;
use crate::model::core::{
    AbstractCityObject, AsAbstractCityObject, AsAbstractCityObjectMut, SpaceBoundaryKind,
};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum CityObjectKind {
    SpaceKind(SpaceKind),
    SpaceBoundaryKind(SpaceBoundaryKind),
}

impl CityObjectKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            CityObjectKind::SpaceKind(x) => x.iter_features(),
            CityObjectKind::SpaceBoundaryKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            CityObjectKind::SpaceKind(x) => x.for_each_feature_mut(f),
            CityObjectKind::SpaceBoundaryKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            CityObjectKind::SpaceKind(x) => x.compute_envelope(),
            CityObjectKind::SpaceBoundaryKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            CityObjectKind::SpaceKind(x) => x.recompute_bounding_shape(),
            CityObjectKind::SpaceBoundaryKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            CityObjectKind::SpaceKind(x) => x.apply_transform(m),
            CityObjectKind::SpaceBoundaryKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractCityObject for CityObjectKind {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::SpaceKind(x) => x.abstract_city_object(),
            Self::SpaceBoundaryKind(x) => x.abstract_city_object(),
        }
    }
}

impl AsAbstractCityObjectMut for CityObjectKind {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject {
        match self {
            Self::SpaceKind(x) => x.abstract_city_object_mut(),
            Self::SpaceBoundaryKind(x) => x.abstract_city_object_mut(),
        }
    }
}

impl_abstract_city_object_traits!(CityObjectKind);
impl_abstract_city_object_mut_traits!(CityObjectKind);

impl HasFeatureType for CityObjectKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::SpaceKind(x) => x.feature_type(),
            Self::SpaceBoundaryKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_city_object_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::CityObjectKind {
            fn from(x: $type) -> Self {
                $crate::model::core::CityObjectKind::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_city_object_kind!($variant, $variant);
    };
}
impl_from_for_city_object_kind!(SpaceKind);
impl_from_for_city_object_kind!(SpaceBoundaryKind);

#[macro_export]
macro_rules! impl_try_from_for_city_object_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::CityObjectKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::CityObjectKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::CityObjectKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind!(CityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_city_object_kind!($variant, $variant);
    };
}
impl_try_from_for_city_object_kind!(SpaceKind);
impl_try_from_for_city_object_kind!(SpaceBoundaryKind);
