use crate::impl_abstract_city_object_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
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
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            CityObjectKind::SpaceKind(x) => x.iter_features(),
            CityObjectKind::SpaceBoundaryKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
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

impl<'a> From<&'a CityObjectKind> for FeatureRef<'a> {
    fn from(item: &'a CityObjectKind) -> Self {
        match item {
            CityObjectKind::SpaceKind(x) => x.into(),
            CityObjectKind::SpaceBoundaryKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a CityObjectKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a CityObjectKind) -> Result<Self, ()> {
        match item {
            CityObjectKind::SpaceKind(x) => x.try_into(),
            CityObjectKind::SpaceBoundaryKind(x) => x.try_into(),
        }
    }
}

impl<'a> From<&'a mut CityObjectKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut CityObjectKind) -> Self {
        match item {
            CityObjectKind::SpaceKind(x) => x.into(),
            CityObjectKind::SpaceBoundaryKind(x) => x.into(),
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
