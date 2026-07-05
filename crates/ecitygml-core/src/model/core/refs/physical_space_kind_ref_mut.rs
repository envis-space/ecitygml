use crate::impl_try_from_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::OccupiedSpaceKind;
use crate::model::core::UnoccupiedSpaceKind;
use crate::model::core::refs::{OccupiedSpaceKindRefMut, UnoccupiedSpaceKindRefMut};
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut, PhysicalSpaceKind,
};

#[derive(Debug)]
pub enum PhysicalSpaceKindRefMut<'a> {
    OccupiedSpaceKind(OccupiedSpaceKindRefMut<'a>),
    UnoccupiedSpaceKind(UnoccupiedSpaceKindRefMut<'a>),
}

impl<'a> From<&'a mut PhysicalSpaceKind> for PhysicalSpaceKindRefMut<'a> {
    fn from(item: &'a mut PhysicalSpaceKind) -> Self {
        match item {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => Self::OccupiedSpaceKind(x.into()),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => Self::UnoccupiedSpaceKind(x.into()),
        }
    }
}

impl<'a> AsAbstractPhysicalSpace for PhysicalSpaceKindRefMut<'a> {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        match self {
            Self::OccupiedSpaceKind(x) => x.abstract_physical_space(),
            Self::UnoccupiedSpaceKind(x) => x.abstract_physical_space(),
        }
    }
}

impl<'a> AsAbstractPhysicalSpaceMut for PhysicalSpaceKindRefMut<'a> {
    fn abstract_physical_space_mut(&mut self) -> &mut AbstractPhysicalSpace {
        match self {
            Self::OccupiedSpaceKind(x) => x.abstract_physical_space_mut(),
            Self::UnoccupiedSpaceKind(x) => x.abstract_physical_space_mut(),
        }
    }
}
crate::impl_abstract_physical_space_traits!(PhysicalSpaceKindRefMut<'_>);
crate::impl_abstract_physical_space_mut_traits!(PhysicalSpaceKindRefMut<'_>);

impl<'a> PhysicalSpaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::OccupiedSpaceKind(x) => x.recompute_bounding_shape(),
            Self::UnoccupiedSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for PhysicalSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::OccupiedSpaceKind(x) => x.feature_type(),
            Self::UnoccupiedSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_physical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::PhysicalSpaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::PhysicalSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind_ref_mut!(PhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_physical_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_physical_space_kind_ref_mut!(OccupiedSpaceKind);
impl_from_for_physical_space_kind_ref_mut!(UnoccupiedSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_physical_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::PhysicalSpaceKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::PhysicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::PhysicalSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind_ref_mut!(PhysicalSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_physical_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::PhysicalSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::PhysicalSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::PhysicalSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_kind_ref_mut_for_enum!(PhysicalSpaceKind, $EnumRefMut);
    };
}
impl_try_from_space_kind_ref_mut_for_enum!(PhysicalSpaceKind, PhysicalSpaceKindRefMut);
