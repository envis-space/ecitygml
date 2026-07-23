use crate::impl_try_from_feature_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AbstractAppearanceKindRef, AbstractCityObjectKindRef};
use crate::model::core::{
    AbstractAppearanceKind, AbstractCityObjectKind, AbstractFeatureWithLifespan,
    AbstractFeatureWithLifespanKind, AsAbstractFeatureWithLifespan, CityModel,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractFeatureWithLifespanKindRef<'a> {
    AbstractAppearanceKind(AbstractAppearanceKindRef<'a>),
    CityModel(&'a CityModel),
    AbstractCityObjectKind(AbstractCityObjectKindRef<'a>),
}

impl<'a> From<&'a AbstractFeatureWithLifespanKind> for AbstractFeatureWithLifespanKindRef<'a> {
    fn from(item: &'a AbstractFeatureWithLifespanKind) -> Self {
        match item {
            AbstractFeatureWithLifespanKind::AbstractAppearanceKind(x) => {
                Self::AbstractAppearanceKind(x.into())
            }
            AbstractFeatureWithLifespanKind::CityModel(x) => Self::CityModel(x),
            AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x) => {
                Self::AbstractCityObjectKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for AbstractFeatureWithLifespanKindRef<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            Self::AbstractAppearanceKind(x) => x.abstract_feature_with_lifespan(),
            Self::CityModel(x) => x.abstract_feature_with_lifespan(),
            Self::AbstractCityObjectKind(x) => x.abstract_feature_with_lifespan(),
        }
    }
}
crate::impl_abstract_feature_with_lifespan_traits!(AbstractFeatureWithLifespanKindRef<'_>);

impl<'a> HasFeatureType for AbstractFeatureWithLifespanKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractAppearanceKind(x) => x.feature_type(),
            Self::CityModel(_) => FeatureType::CityModel,
            Self::AbstractCityObjectKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_with_lifespan_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::core::refs::AbstractFeatureWithLifespanKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractFeatureWithLifespanKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref!(AbstractFeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_with_lifespan_kind_ref!($variant, $variant);
    };
}
impl_from_for_feature_with_lifespan_kind_ref!(AbstractAppearanceKind);
impl_from_for_feature_with_lifespan_kind_ref!(CityModel);
impl_from_for_feature_with_lifespan_kind_ref!(AbstractCityObjectKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_with_lifespan_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureWithLifespanKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureWithLifespanKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureWithLifespanKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref!(AbstractFeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_feature_with_lifespan_kind_ref!(CityModel);

#[macro_export]
macro_rules! impl_try_from_feature_with_lifespan_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureWithLifespanKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureWithLifespanKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureWithLifespanKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_for_enum!(AbstractFeatureWithLifespanKind, $EnumRef);
    };
}
impl_try_from_feature_kind_ref_for_enum!(
    AbstractFeatureWithLifespanKind,
    AbstractFeatureWithLifespanKindRef
);
