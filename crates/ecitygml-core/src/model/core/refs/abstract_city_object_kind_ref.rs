use crate::impl_try_from_feature_with_lifespan_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AbstractSpaceBoundaryKindRef, AbstractSpaceKindRef};
use crate::model::core::{AbstractCityObject, AbstractCityObjectKind, AsAbstractCityObject};

#[derive(Debug, Clone, Copy)]
pub enum AbstractCityObjectKindRef<'a> {
    AbstractSpaceKind(AbstractSpaceKindRef<'a>),
    AbstractSpaceBoundaryKind(AbstractSpaceBoundaryKindRef<'a>),
}

impl<'a> From<&'a AbstractCityObjectKind> for AbstractCityObjectKindRef<'a> {
    fn from(item: &'a AbstractCityObjectKind) -> Self {
        match item {
            AbstractCityObjectKind::AbstractSpaceKind(x) => Self::AbstractSpaceKind(x.into()),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => {
                Self::AbstractSpaceBoundaryKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractCityObject for AbstractCityObjectKindRef<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::AbstractSpaceKind(x) => x.abstract_city_object(),
            Self::AbstractSpaceBoundaryKind(x) => x.abstract_city_object(),
        }
    }
}
crate::impl_abstract_city_object_traits!(AbstractCityObjectKindRef<'_>);

impl<'a> HasFeatureType for AbstractCityObjectKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractSpaceKind(x) => x.feature_type(),
            Self::AbstractSpaceBoundaryKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_city_object_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractCityObjectKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractCityObjectKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind_ref!(AbstractCityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_city_object_kind_ref!($variant, $crate::model::core::$variant);
    };
}
impl_from_for_city_object_kind_ref!(AbstractSpaceKind);
impl_from_for_city_object_kind_ref!(AbstractSpaceBoundaryKind);

#[macro_export]
macro_rules! impl_try_from_for_city_object_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractCityObjectKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractCityObjectKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractCityObjectKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref!(AbstractCityObjectKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_city_object_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractCityObjectKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractCityObjectKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractCityObjectKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_with_lifespan_kind_ref_for_enum!(
            AbstractCityObjectKind,
            $EnumRef
        );
    };
}
impl_try_from_feature_with_lifespan_kind_ref_for_enum!(
    AbstractCityObjectKind,
    AbstractCityObjectKindRef
);
