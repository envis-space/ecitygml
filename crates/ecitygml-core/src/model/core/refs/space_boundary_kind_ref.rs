use crate::impl_try_from_city_object_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::ThematicSurfaceKindRef;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, SpaceBoundaryKind, ThematicSurfaceKind,
};
use crate::model::relief::refs::ReliefComponentKindRef;
use crate::model::relief::{ReliefComponentKind, ReliefFeature};

#[derive(Debug, Clone, Copy)]
pub enum SpaceBoundaryKindRef<'a> {
    ThematicSurfaceKind(ThematicSurfaceKindRef<'a>),
    ReliefFeature(&'a ReliefFeature),
    ReliefComponentKind(ReliefComponentKindRef<'a>),
}

impl<'a> From<&'a SpaceBoundaryKind> for SpaceBoundaryKindRef<'a> {
    fn from(item: &'a SpaceBoundaryKind) -> Self {
        match item {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => Self::ThematicSurfaceKind(x.into()),
            SpaceBoundaryKind::ReliefFeature(x) => Self::ReliefFeature(x),
            SpaceBoundaryKind::ReliefComponentKind(x) => Self::ReliefComponentKind(x.into()),
        }
    }
}

impl<'a> AsAbstractSpaceBoundary for SpaceBoundaryKindRef<'a> {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        match self {
            Self::ThematicSurfaceKind(x) => x.abstract_space_boundary(),
            Self::ReliefFeature(x) => x.abstract_space_boundary(),
            Self::ReliefComponentKind(x) => x.abstract_space_boundary(),
        }
    }
}
crate::impl_abstract_space_boundary_traits!(SpaceBoundaryKindRef<'_>);

impl<'a> HasFeatureType for SpaceBoundaryKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::ThematicSurfaceKind(x) => x.feature_type(),
            Self::ReliefFeature(_) => FeatureType::ReliefFeature,
            Self::ReliefComponentKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_boundary_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::SpaceBoundaryKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::SpaceBoundaryKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref!(SpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_boundary_kind_ref!($variant, $variant);
    };
}
impl_from_for_space_boundary_kind_ref!(ThematicSurfaceKind);
impl_from_for_space_boundary_kind_ref!(ReliefFeature);
impl_from_for_space_boundary_kind_ref!(ReliefComponentKind);

#[macro_export]
macro_rules! impl_try_from_for_space_boundary_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceBoundaryKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::SpaceBoundaryKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceBoundaryKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref!(SpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_boundary_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_space_boundary_kind_ref!(ReliefFeature);

#[macro_export]
macro_rules! impl_try_from_space_boundary_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceBoundaryKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::SpaceBoundaryKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceBoundaryKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_for_enum!(SpaceBoundaryKind, $EnumRef);
    };
}
impl_try_from_city_object_kind_ref_for_enum!(SpaceBoundaryKind, SpaceBoundaryKindRef);
