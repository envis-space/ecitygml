use crate::impl_try_from_feature_with_lifespan_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{SpaceBoundaryKindRef, SpaceKindRef};
use crate::model::core::{AbstractCityObject, AsAbstractCityObject, CityObjectKind};

#[derive(Debug, Clone, Copy)]
pub enum CityObjectKindRef<'a> {
    SpaceKind(SpaceKindRef<'a>),
    SpaceBoundaryKind(SpaceBoundaryKindRef<'a>),
}

impl<'a> From<&'a CityObjectKind> for CityObjectKindRef<'a> {
    fn from(item: &'a CityObjectKind) -> Self {
        match item {
            CityObjectKind::SpaceKind(x) => Self::SpaceKind(x.into()),
            CityObjectKind::SpaceBoundaryKind(x) => Self::SpaceBoundaryKind(x.into()),
        }
    }
}

impl<'a> AsAbstractCityObject for CityObjectKindRef<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::SpaceKind(x) => x.abstract_city_object(),
            Self::SpaceBoundaryKind(x) => x.abstract_city_object(),
        }
    }
}
crate::impl_abstract_city_object_traits!(CityObjectKindRef<'_>);

impl<'a> HasFeatureType for CityObjectKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::SpaceKind(x) => x.feature_type(),
            Self::SpaceBoundaryKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_city_object_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::CityObjectKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::CityObjectKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind_ref!(CityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_city_object_kind_ref!($variant, $crate::model::core::$variant);
    };
}
impl_from_for_city_object_kind_ref!(SpaceKind);
impl_from_for_city_object_kind_ref!(SpaceBoundaryKind);

#[macro_export]
macro_rules! impl_try_from_for_city_object_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::CityObjectKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::CityObjectKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::CityObjectKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref!(CityObjectKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_city_object_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::CityObjectKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::CityObjectKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::CityObjectKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_with_lifespan_kind_ref_for_enum!(CityObjectKind, $EnumRef);
    };
}
impl_try_from_feature_with_lifespan_kind_ref_for_enum!(CityObjectKind, CityObjectKindRef);
