use crate::impl_try_from_city_object_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::LogicalSpaceKind;
use crate::model::core::PhysicalSpaceKind;
use crate::model::core::refs::{LogicalSpaceKindRefMut, PhysicalSpaceKindRefMut};
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut, SpaceKind};

#[derive(Debug)]
pub enum SpaceKindRefMut<'a> {
    LogicalSpaceKind(LogicalSpaceKindRefMut<'a>),
    PhysicalSpaceKind(PhysicalSpaceKindRefMut<'a>),
}

impl<'a> From<&'a mut SpaceKind> for SpaceKindRefMut<'a> {
    fn from(item: &'a mut SpaceKind) -> Self {
        match item {
            SpaceKind::LogicalSpaceKind(x) => Self::LogicalSpaceKind(x.into()),
            SpaceKind::PhysicalSpaceKind(x) => Self::PhysicalSpaceKind(x.into()),
        }
    }
}

impl<'a> AsAbstractSpace for SpaceKindRefMut<'a> {
    fn abstract_space(&self) -> &AbstractSpace {
        match self {
            Self::LogicalSpaceKind(x) => x.abstract_space(),
            Self::PhysicalSpaceKind(x) => x.abstract_space(),
        }
    }
}

impl<'a> AsAbstractSpaceMut for SpaceKindRefMut<'a> {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        match self {
            Self::LogicalSpaceKind(x) => x.abstract_space_mut(),
            Self::PhysicalSpaceKind(x) => x.abstract_space_mut(),
        }
    }
}
crate::impl_abstract_space_traits!(SpaceKindRefMut<'_>);
crate::impl_abstract_space_mut_traits!(SpaceKindRefMut<'_>);

impl<'a> SpaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::LogicalSpaceKind(x) => x.recompute_bounding_shape(),
            Self::PhysicalSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for SpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::LogicalSpaceKind(x) => x.feature_type(),
            Self::PhysicalSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::SpaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::SpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref_mut!(SpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_space_kind_ref_mut!(LogicalSpaceKind);
impl_from_for_space_kind_ref_mut!(PhysicalSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::SpaceKindRefMut<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref_mut!(SpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceKindRefMut<'a>> for $EnumRefMut<'a> {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::SpaceKindRefMut<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_mut_for_enum!(SpaceKind, $EnumRefMut);
    };
}
impl_try_from_city_object_kind_ref_mut_for_enum!(SpaceKind, SpaceKindRefMut);
