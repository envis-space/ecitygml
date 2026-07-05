use crate::impl_try_from_feature_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AppearanceKindRef, CityObjectKindRef};
use crate::model::core::{
    AbstractFeatureWithLifespan, AppearanceKind, AsAbstractFeatureWithLifespan, CityModel,
    CityObjectKind, FeatureWithLifespanKind,
};

#[derive(Debug, Clone, Copy)]
pub enum FeatureWithLifespanKindRef<'a> {
    AppearanceKind(AppearanceKindRef<'a>),
    CityModel(&'a CityModel),
    CityObjectKind(CityObjectKindRef<'a>),
}

impl<'a> From<&'a FeatureWithLifespanKind> for FeatureWithLifespanKindRef<'a> {
    fn from(item: &'a FeatureWithLifespanKind) -> Self {
        match item {
            FeatureWithLifespanKind::AppearanceKind(x) => Self::AppearanceKind(x.into()),
            FeatureWithLifespanKind::CityModel(x) => Self::CityModel(x),
            FeatureWithLifespanKind::CityObjectKind(x) => Self::CityObjectKind(x.into()),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for FeatureWithLifespanKindRef<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            Self::AppearanceKind(x) => x.abstract_feature_with_lifespan(),
            Self::CityModel(x) => x.abstract_feature_with_lifespan(),
            Self::CityObjectKind(x) => x.abstract_feature_with_lifespan(),
        }
    }
}
crate::impl_abstract_feature_with_lifespan_traits!(FeatureWithLifespanKindRef<'_>);

impl<'a> HasFeatureType for FeatureWithLifespanKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AppearanceKind(x) => x.feature_type(),
            Self::CityModel(_) => unreachable!(),
            Self::CityObjectKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_with_lifespan_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::FeatureWithLifespanKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::FeatureWithLifespanKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref!(FeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_with_lifespan_kind_ref!($variant, $variant);
    };
}
impl_from_for_feature_with_lifespan_kind_ref!(AppearanceKind);
impl_from_for_feature_with_lifespan_kind_ref!(CityModel);
impl_from_for_feature_with_lifespan_kind_ref!(CityObjectKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_with_lifespan_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureWithLifespanKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::FeatureWithLifespanKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureWithLifespanKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref!(FeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_feature_with_lifespan_kind_ref!(CityModel);

#[macro_export]
macro_rules! impl_try_from_feature_with_lifespan_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureWithLifespanKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::FeatureWithLifespanKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureWithLifespanKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_for_enum!(FeatureWithLifespanKind, $EnumRef);
    };
}
impl_try_from_feature_kind_ref_for_enum!(FeatureWithLifespanKind, FeatureWithLifespanKindRef);
