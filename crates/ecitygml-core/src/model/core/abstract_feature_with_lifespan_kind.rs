use crate::impl_abstract_feature_with_lifespan_mut_traits;
use crate::impl_abstract_feature_with_lifespan_traits;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::abstract_city_object_kind::AbstractCityObjectKind;
use crate::model::core::{
    AbstractAppearanceKind, AbstractFeatureWithLifespan, AsAbstractFeature,
    AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut, CityModel,
};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractFeatureWithLifespanKind {
    AbstractAppearanceKind(AbstractAppearanceKind),
    CityModel(CityModel),
    AbstractCityObjectKind(AbstractCityObjectKind),
}

impl AsAbstractFeatureWithLifespan for AbstractFeatureWithLifespanKind {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            AbstractFeatureWithLifespanKind::AbstractAppearanceKind(x) => {
                x.abstract_feature_with_lifespan()
            }
            AbstractFeatureWithLifespanKind::CityModel(x) => x.abstract_feature_with_lifespan(),
            AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x) => {
                x.abstract_feature_with_lifespan()
            }
        }
    }
}

impl AsAbstractFeatureWithLifespanMut for AbstractFeatureWithLifespanKind {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        match self {
            AbstractFeatureWithLifespanKind::AbstractAppearanceKind(x) => {
                x.abstract_feature_with_lifespan_mut()
            }
            AbstractFeatureWithLifespanKind::CityModel(x) => x.abstract_feature_with_lifespan_mut(),
            AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x) => {
                x.abstract_feature_with_lifespan_mut()
            }
        }
    }
}

impl_abstract_feature_with_lifespan_traits!(AbstractFeatureWithLifespanKind);
impl_abstract_feature_with_lifespan_mut_traits!(AbstractFeatureWithLifespanKind);

impl HasFeatureType for AbstractFeatureWithLifespanKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractAppearanceKind(x) => x.feature_type(),
            Self::CityModel(x) => x.feature_type(),
            Self::AbstractCityObjectKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_with_lifespan_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractFeatureWithLifespanKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractFeatureWithLifespanKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind!(AbstractFeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_with_lifespan_kind!($variant, $variant);
    };
}
impl_from_for_feature_with_lifespan_kind!(AbstractAppearanceKind);
impl_from_for_feature_with_lifespan_kind!(CityModel);
impl_from_for_feature_with_lifespan_kind!(AbstractCityObjectKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_with_lifespan_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractFeatureWithLifespanKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::AbstractFeatureWithLifespanKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractFeatureWithLifespanKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind!(AbstractFeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_with_lifespan_kind!($variant, $variant);
    };
}
