use crate::impl_try_from_city_object_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{LogicalSpaceKindRef, PhysicalSpaceKindRef};
use crate::model::core::{
    AbstractSpace, AsAbstractSpace, LogicalSpaceKind, PhysicalSpaceKind, SpaceKind,
};

#[derive(Debug, Clone, Copy)]
pub enum SpaceKindRef<'a> {
    LogicalSpaceKind(LogicalSpaceKindRef<'a>),
    PhysicalSpaceKind(PhysicalSpaceKindRef<'a>),
}

impl<'a> From<&'a SpaceKind> for SpaceKindRef<'a> {
    fn from(item: &'a SpaceKind) -> Self {
        match item {
            SpaceKind::LogicalSpaceKind(x) => Self::LogicalSpaceKind(x.into()),
            SpaceKind::PhysicalSpaceKind(x) => Self::PhysicalSpaceKind(x.into()),
        }
    }
}

impl<'a> AsAbstractSpace for SpaceKindRef<'a> {
    fn abstract_space(&self) -> &AbstractSpace {
        match self {
            Self::LogicalSpaceKind(x) => x.abstract_space(),
            Self::PhysicalSpaceKind(x) => x.abstract_space(),
        }
    }
}
crate::impl_abstract_space_traits!(SpaceKindRef<'_>);

impl<'a> HasFeatureType for SpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::LogicalSpaceKind(x) => x.feature_type(),
            Self::PhysicalSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::SpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::SpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref!(SpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_space_kind_ref!(LogicalSpaceKind);
impl_from_for_space_kind_ref!(PhysicalSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::SpaceKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref!(SpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::SpaceKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_for_enum!(SpaceKind, $EnumRef);
    };
}
impl_try_from_city_object_kind_ref_for_enum!(SpaceKind, SpaceKindRef);
