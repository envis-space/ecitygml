use crate::impl_try_from_space_kind_ref_mut_for_enum;
use crate::model::building::BuildingSubdivisionKind;
use crate::model::building::refs::BuildingSubdivisionKindRefMut;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut, LogicalSpaceKind,
};

#[derive(Debug)]
pub enum LogicalSpaceKindRefMut<'a> {
    BuildingSubdivisionKind(BuildingSubdivisionKindRefMut<'a>),
}

impl<'a> From<&'a mut LogicalSpaceKind> for LogicalSpaceKindRefMut<'a> {
    fn from(item: &'a mut LogicalSpaceKind) -> Self {
        match item {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => Self::BuildingSubdivisionKind(x.into()),
        }
    }
}

impl<'a> AsAbstractLogicalSpace for LogicalSpaceKindRefMut<'a> {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        match self {
            Self::BuildingSubdivisionKind(x) => x.abstract_logical_space(),
        }
    }
}

impl<'a> AsAbstractLogicalSpaceMut for LogicalSpaceKindRefMut<'a> {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace {
        match self {
            Self::BuildingSubdivisionKind(x) => x.abstract_logical_space_mut(),
        }
    }
}
crate::impl_abstract_logical_space_traits!(LogicalSpaceKindRefMut<'_>);
crate::impl_abstract_logical_space_mut_traits!(LogicalSpaceKindRefMut<'_>);

impl<'a> LogicalSpaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::BuildingSubdivisionKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for LogicalSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingSubdivisionKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_logical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::LogicalSpaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::LogicalSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref_mut!(LogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_logical_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_logical_space_kind_ref_mut!(BuildingSubdivisionKind);

#[macro_export]
macro_rules! impl_try_from_for_logical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::LogicalSpaceKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::LogicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::LogicalSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref_mut!(LogicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_logical_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::LogicalSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::LogicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::LogicalSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_mut_for_enum!(LogicalSpaceKind, $EnumRefMut);
    };
}
impl_try_from_space_kind_ref_mut_for_enum!(LogicalSpaceKind, LogicalSpaceKindRefMut);
