use crate::impl_try_from_city_object_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::AbstractLogicalSpaceKind;
use crate::model::core::AbstractPhysicalSpaceKind;
use crate::model::core::refs::{AbstractLogicalSpaceKindRefMut, AbstractPhysicalSpaceKindRefMut};
use crate::model::core::{AbstractSpace, AbstractSpaceKind, AsAbstractSpace, AsAbstractSpaceMut};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractSpaceKindRefMut<'a> {
    AbstractLogicalSpaceKind(AbstractLogicalSpaceKindRefMut<'a>),
    AbstractPhysicalSpaceKind(AbstractPhysicalSpaceKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractSpaceKind> for AbstractSpaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractSpaceKind) -> Self {
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

impl<'a> AsAbstractSpace for AbstractSpaceKindRefMut<'a> {
    fn abstract_space(&self) -> &AbstractSpace {
        match self {
            Self::AbstractLogicalSpaceKind(x) => x.abstract_space(),
            Self::AbstractPhysicalSpaceKind(x) => x.abstract_space(),
        }
    }
}

impl<'a> AsAbstractSpaceMut for AbstractSpaceKindRefMut<'a> {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        match self {
            Self::AbstractLogicalSpaceKind(x) => x.abstract_space_mut(),
            Self::AbstractPhysicalSpaceKind(x) => x.abstract_space_mut(),
        }
    }
}
crate::impl_abstract_space_traits!(AbstractSpaceKindRefMut<'_>);
crate::impl_abstract_space_mut_traits!(AbstractSpaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractLogicalSpaceKind(x) => x.feature_type(),
            Self::AbstractPhysicalSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::AbstractSpaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref_mut!(AbstractSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_space_kind_ref_mut!(AbstractLogicalSpaceKind);
impl_from_for_space_kind_ref_mut!(AbstractPhysicalSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref_mut!(AbstractSpaceKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_mut_for_enum!(AbstractSpaceKind, $EnumRefMut);
    };
}
impl_try_from_city_object_kind_ref_mut_for_enum!(AbstractSpaceKind, AbstractSpaceKindRefMut);

impl<'a> RecomputeBoundingShape for AbstractSpaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractLogicalSpaceKind(x) => x.recompute_bounding_shape(),
            Self::AbstractPhysicalSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}
