use crate::impl_try_from_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{OccupiedSpaceKindRef, UnoccupiedSpaceKindRef};
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, OccupiedSpaceKind, PhysicalSpaceKind,
    UnoccupiedSpaceKind,
};

#[derive(Debug, Clone, Copy)]
pub enum PhysicalSpaceKindRef<'a> {
    OccupiedSpaceKind(OccupiedSpaceKindRef<'a>),
    UnoccupiedSpaceKind(UnoccupiedSpaceKindRef<'a>),
}

impl<'a> From<&'a PhysicalSpaceKind> for PhysicalSpaceKindRef<'a> {
    fn from(item: &'a PhysicalSpaceKind) -> Self {
        match item {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => Self::OccupiedSpaceKind(x.into()),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => Self::UnoccupiedSpaceKind(x.into()),
        }
    }
}

impl<'a> AsAbstractPhysicalSpace for PhysicalSpaceKindRef<'a> {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        match self {
            Self::OccupiedSpaceKind(x) => x.abstract_physical_space(),
            Self::UnoccupiedSpaceKind(x) => x.abstract_physical_space(),
        }
    }
}
crate::impl_abstract_physical_space_traits!(PhysicalSpaceKindRef<'_>);

impl<'a> HasFeatureType for PhysicalSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::OccupiedSpaceKind(x) => x.feature_type(),
            Self::UnoccupiedSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_physical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::PhysicalSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::PhysicalSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref!(PhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_physical_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_physical_space_kind_ref!(OccupiedSpaceKind);
impl_from_for_physical_space_kind_ref!(UnoccupiedSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_physical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::PhysicalSpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::PhysicalSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::PhysicalSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref!(PhysicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_physical_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::PhysicalSpaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::PhysicalSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::PhysicalSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_for_enum!(PhysicalSpaceKind, $EnumRef);
    };
}
impl_try_from_space_kind_ref_for_enum!(PhysicalSpaceKind, PhysicalSpaceKindRef);
