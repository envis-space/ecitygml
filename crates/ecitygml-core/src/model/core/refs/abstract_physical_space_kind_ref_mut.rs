use crate::impl_try_from_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::AbstractOccupiedSpaceKind;
use crate::model::core::AbstractUnoccupiedSpaceKind;
use crate::model::core::refs::{
    AbstractOccupiedSpaceKindRefMut, AbstractUnoccupiedSpaceKindRefMut,
};
use crate::model::core::{
    AbstractPhysicalSpace, AbstractPhysicalSpaceKind, AsAbstractPhysicalSpace,
    AsAbstractPhysicalSpaceMut,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractPhysicalSpaceKindRefMut<'a> {
    AbstractOccupiedSpaceKind(AbstractOccupiedSpaceKindRefMut<'a>),
    AbstractUnoccupiedSpaceKind(AbstractUnoccupiedSpaceKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractPhysicalSpaceKind> for AbstractPhysicalSpaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractPhysicalSpaceKind) -> Self {
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

impl<'a> AsAbstractPhysicalSpace for AbstractPhysicalSpaceKindRefMut<'a> {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        match self {
            Self::AbstractOccupiedSpaceKind(x) => x.abstract_physical_space(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.abstract_physical_space(),
        }
    }
}

impl<'a> AsAbstractPhysicalSpaceMut for AbstractPhysicalSpaceKindRefMut<'a> {
    fn abstract_physical_space_mut(&mut self) -> &mut AbstractPhysicalSpace {
        match self {
            Self::AbstractOccupiedSpaceKind(x) => x.abstract_physical_space_mut(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.abstract_physical_space_mut(),
        }
    }
}
crate::impl_abstract_physical_space_traits!(AbstractPhysicalSpaceKindRefMut<'_>);
crate::impl_abstract_physical_space_mut_traits!(AbstractPhysicalSpaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractPhysicalSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractOccupiedSpaceKind(x) => x.feature_type(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_physical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractPhysicalSpaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractPhysicalSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref_mut!(AbstractPhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_physical_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_physical_space_kind_ref_mut!(AbstractOccupiedSpaceKind);
impl_from_for_physical_space_kind_ref_mut!(AbstractUnoccupiedSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_physical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractPhysicalSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractPhysicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractPhysicalSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref_mut!(AbstractPhysicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_physical_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractPhysicalSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractPhysicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractPhysicalSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_mut_for_enum!(AbstractPhysicalSpaceKind, $EnumRefMut);
    };
}
impl_try_from_space_kind_ref_mut_for_enum!(
    AbstractPhysicalSpaceKind,
    AbstractPhysicalSpaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractPhysicalSpaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractOccupiedSpaceKind(x) => x.recompute_bounding_shape(),
            Self::AbstractUnoccupiedSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}
