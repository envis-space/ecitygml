use crate::impl_try_from_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AbstractOccupiedSpaceKindRef, AbstractUnoccupiedSpaceKindRef};
use crate::model::core::{
    AbstractOccupiedSpaceKind, AbstractPhysicalSpace, AbstractPhysicalSpaceKind,
    AbstractUnoccupiedSpaceKind, AsAbstractPhysicalSpace,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractPhysicalSpaceKindRef<'a> {
    AbstractOccupiedSpaceKind(AbstractOccupiedSpaceKindRef<'a>),
    AbstractUnoccupiedSpaceKind(AbstractUnoccupiedSpaceKindRef<'a>),
}

impl<'a> From<&'a AbstractPhysicalSpaceKind> for AbstractPhysicalSpaceKindRef<'a> {
    fn from(item: &'a AbstractPhysicalSpaceKind) -> Self {
        match item {
            AbstractPhysicalSpaceKind::AbstractOccupiedSpaceKind(x) => {
                Self::AbstractOccupiedSpaceKind(x.into())
            }
            AbstractPhysicalSpaceKind::AbstractUnoccupiedSpaceKind(x) => {
                Self::AbstractUnoccupiedSpaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractPhysicalSpace for AbstractPhysicalSpaceKindRef<'a> {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        match self {
            Self::AbstractOccupiedSpaceKind(x) => x.abstract_physical_space(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.abstract_physical_space(),
        }
    }
}
crate::impl_abstract_physical_space_traits!(AbstractPhysicalSpaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractPhysicalSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractOccupiedSpaceKind(x) => x.feature_type(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_physical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractPhysicalSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractPhysicalSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref!(AbstractPhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_physical_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_physical_space_kind_ref!(AbstractOccupiedSpaceKind);
impl_from_for_physical_space_kind_ref!(AbstractUnoccupiedSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_physical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractPhysicalSpaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractPhysicalSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractPhysicalSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref!(AbstractPhysicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_physical_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractPhysicalSpaceKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractPhysicalSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractPhysicalSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_for_enum!(AbstractPhysicalSpaceKind, $EnumRef);
    };
}
impl_try_from_space_kind_ref_for_enum!(AbstractPhysicalSpaceKind, AbstractPhysicalSpaceKindRef);
