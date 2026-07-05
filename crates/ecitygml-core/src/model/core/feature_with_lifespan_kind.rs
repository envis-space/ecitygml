use crate::impl_abstract_feature_with_lifespan_mut_traits;
use crate::impl_abstract_feature_with_lifespan_traits;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::city_object_kind::CityObjectKind;
use crate::model::core::{
    AbstractFeatureWithLifespan, AppearanceKind, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut, CityModel,
};

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureWithLifespanKind {
    AppearanceKind(AppearanceKind),
    CityModel(CityModel),
    CityObjectKind(CityObjectKind),
}

impl AsAbstractFeatureWithLifespan for FeatureWithLifespanKind {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            FeatureWithLifespanKind::AppearanceKind(x) => x.abstract_feature_with_lifespan(),
            FeatureWithLifespanKind::CityModel(x) => x.abstract_feature_with_lifespan(),
            FeatureWithLifespanKind::CityObjectKind(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl AsAbstractFeatureWithLifespanMut for FeatureWithLifespanKind {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        match self {
            FeatureWithLifespanKind::AppearanceKind(x) => x.abstract_feature_with_lifespan_mut(),
            FeatureWithLifespanKind::CityModel(x) => x.abstract_feature_with_lifespan_mut(),
            FeatureWithLifespanKind::CityObjectKind(x) => x.abstract_feature_with_lifespan_mut(),
        }
    }
}

impl_abstract_feature_with_lifespan_traits!(FeatureWithLifespanKind);
impl_abstract_feature_with_lifespan_mut_traits!(FeatureWithLifespanKind);

impl HasFeatureType for FeatureWithLifespanKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AppearanceKind(x) => x.feature_type(),
            Self::CityModel(_) => unreachable!(),
            Self::CityObjectKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_try_from_for_feature_with_lifespan_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::FeatureWithLifespanKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::FeatureWithLifespanKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::FeatureWithLifespanKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind!(FeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_with_lifespan_kind!($variant, $variant);
    };
}
