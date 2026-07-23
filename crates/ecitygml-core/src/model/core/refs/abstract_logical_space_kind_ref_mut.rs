use crate::impl_try_from_space_kind_ref_mut_for_enum;
use crate::model::building::AbstractBuildingSubdivisionKind;
use crate::model::building::refs::AbstractBuildingSubdivisionKindRefMut;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractLogicalSpace, AbstractLogicalSpaceKind, AsAbstractLogicalSpace,
    AsAbstractLogicalSpaceMut,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractLogicalSpaceKindRefMut<'a> {
    AbstractBuildingSubdivisionKind(AbstractBuildingSubdivisionKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractLogicalSpaceKind> for AbstractLogicalSpaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractLogicalSpaceKind) -> Self {
        match item {
            AbstractLogicalSpaceKind::AbstractBuildingSubdivisionKind(x) => {
                Self::AbstractBuildingSubdivisionKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractLogicalSpace for AbstractLogicalSpaceKindRefMut<'a> {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        match self {
            Self::AbstractBuildingSubdivisionKind(x) => x.abstract_logical_space(),
        }
    }
}

impl<'a> AsAbstractLogicalSpaceMut for AbstractLogicalSpaceKindRefMut<'a> {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace {
        match self {
            Self::AbstractBuildingSubdivisionKind(x) => x.abstract_logical_space_mut(),
        }
    }
}
crate::impl_abstract_logical_space_traits!(AbstractLogicalSpaceKindRefMut<'_>);
crate::impl_abstract_logical_space_mut_traits!(AbstractLogicalSpaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractLogicalSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractBuildingSubdivisionKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_logical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractLogicalSpaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractLogicalSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref_mut!(AbstractLogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_logical_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_logical_space_kind_ref_mut!(AbstractBuildingSubdivisionKind);

#[macro_export]
macro_rules! impl_try_from_for_logical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractLogicalSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractLogicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractLogicalSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref_mut!(AbstractLogicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_logical_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractLogicalSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractLogicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractLogicalSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_mut_for_enum!(AbstractLogicalSpaceKind, $EnumRefMut);
    };
}
impl_try_from_space_kind_ref_mut_for_enum!(
    AbstractLogicalSpaceKind,
    AbstractLogicalSpaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractLogicalSpaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractBuildingSubdivisionKind(x) => x.recompute_bounding_shape(),
        }
    }
}
