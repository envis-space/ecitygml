use crate::impl_try_from_space_kind_ref_for_enum;
use crate::model::building::BuildingSubdivisionKind;
use crate::model::building::refs::BuildingSubdivisionKindRef;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, LogicalSpaceKind};

#[derive(Debug, Clone, Copy)]
pub enum LogicalSpaceKindRef<'a> {
    BuildingSubdivisionKind(BuildingSubdivisionKindRef<'a>),
}

impl<'a> From<&'a LogicalSpaceKind> for LogicalSpaceKindRef<'a> {
    fn from(item: &'a LogicalSpaceKind) -> Self {
        match item {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => Self::BuildingSubdivisionKind(x.into()),
        }
    }
}

impl<'a> AsAbstractLogicalSpace for LogicalSpaceKindRef<'a> {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        match self {
            Self::BuildingSubdivisionKind(x) => x.abstract_logical_space(),
        }
    }
}
crate::impl_abstract_logical_space_traits!(LogicalSpaceKindRef<'_>);

impl<'a> HasFeatureType for LogicalSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::BuildingSubdivisionKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_logical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::LogicalSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::LogicalSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref!(LogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_logical_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_logical_space_kind_ref!(BuildingSubdivisionKind);

#[macro_export]
macro_rules! impl_try_from_for_logical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::LogicalSpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::LogicalSpaceKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::LogicalSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref!(LogicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_logical_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::LogicalSpaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::LogicalSpaceKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::LogicalSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_for_enum!(LogicalSpaceKind, $EnumRef);
    };
}
impl_try_from_space_kind_ref_for_enum!(LogicalSpaceKind, LogicalSpaceKindRef);
