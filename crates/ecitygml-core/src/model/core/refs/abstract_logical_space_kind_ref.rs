use crate::impl_try_from_space_kind_ref_for_enum;
use crate::model::building::AbstractBuildingSubdivisionKind;
use crate::model::building::refs::AbstractBuildingSubdivisionKindRef;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{AbstractLogicalSpace, AbstractLogicalSpaceKind, AsAbstractLogicalSpace};

#[derive(Debug, Clone, Copy)]
pub enum AbstractLogicalSpaceKindRef<'a> {
    AbstractBuildingSubdivisionKind(AbstractBuildingSubdivisionKindRef<'a>),
}

impl<'a> From<&'a AbstractLogicalSpaceKind> for AbstractLogicalSpaceKindRef<'a> {
    fn from(item: &'a AbstractLogicalSpaceKind) -> Self {
        match item {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                Self::AbstractBuildingSubdivisionKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractLogicalSpace for AbstractLogicalSpaceKindRef<'a> {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        match self {
            Self::AbstractBuildingSubdivisionKind(x) => x.abstract_logical_space(),
        }
    }
}
crate::impl_abstract_logical_space_traits!(AbstractLogicalSpaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractLogicalSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractBuildingSubdivisionKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_logical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractLogicalSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractLogicalSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref!(AbstractLogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_logical_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_logical_space_kind_ref!(AbstractBuildingSubdivisionKind);

#[macro_export]
macro_rules! impl_try_from_for_logical_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractLogicalSpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractLogicalSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractLogicalSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref!(AbstractLogicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_logical_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractLogicalSpaceKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractLogicalSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractLogicalSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_for_enum!(AbstractLogicalSpaceKind, $EnumRef);
    };
}
impl_try_from_space_kind_ref_for_enum!(AbstractLogicalSpaceKind, AbstractLogicalSpaceKindRef);
