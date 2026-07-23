use crate::impl_try_from_city_object_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AbstractLogicalSpaceKindRef, AbstractPhysicalSpaceKindRef};
use crate::model::core::{
    AbstractLogicalSpaceKind, AbstractPhysicalSpaceKind, AbstractSpace, AbstractSpaceKind,
    AsAbstractSpace,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractSpaceKindRef<'a> {
    AbstractLogicalSpaceKind(AbstractLogicalSpaceKindRef<'a>),
    AbstractPhysicalSpaceKind(AbstractPhysicalSpaceKindRef<'a>),
}

impl<'a> From<&'a AbstractSpaceKind> for AbstractSpaceKindRef<'a> {
    fn from(item: &'a AbstractSpaceKind) -> Self {
        match item {
            AbstractSpaceKind::AbstractLogicalSpaceKind(x) => {
                Self::AbstractLogicalSpaceKind(x.into())
            }
            AbstractSpaceKind::AbstractPhysicalSpaceKind(x) => {
                Self::AbstractPhysicalSpaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractSpace for AbstractSpaceKindRef<'a> {
    fn abstract_space(&self) -> &AbstractSpace {
        match self {
            Self::AbstractLogicalSpaceKind(x) => x.abstract_space(),
            Self::AbstractPhysicalSpaceKind(x) => x.abstract_space(),
        }
    }
}
crate::impl_abstract_space_traits!(AbstractSpaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractLogicalSpaceKind(x) => x.feature_type(),
            Self::AbstractPhysicalSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref!(AbstractSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_space_kind_ref!(AbstractLogicalSpaceKind);
impl_from_for_space_kind_ref!(AbstractPhysicalSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref!(AbstractSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_for_enum!(AbstractSpaceKind, $EnumRef);
    };
}
impl_try_from_city_object_kind_ref_for_enum!(AbstractSpaceKind, AbstractSpaceKindRef);
