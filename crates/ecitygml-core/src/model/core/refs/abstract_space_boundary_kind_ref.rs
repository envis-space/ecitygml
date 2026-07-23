use crate::impl_try_from_city_object_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::AbstractThematicSurfaceKindRef;
use crate::model::core::{
    AbstractSpaceBoundary, AbstractSpaceBoundaryKind, AbstractThematicSurfaceKind,
    AsAbstractSpaceBoundary,
};
use crate::model::relief::refs::AbstractReliefComponentKindRef;
use crate::model::relief::{AbstractReliefComponentKind, ReliefFeature};

#[derive(Debug, Clone, Copy)]
pub enum AbstractSpaceBoundaryKindRef<'a> {
    AbstractThematicSurfaceKind(AbstractThematicSurfaceKindRef<'a>),
    ReliefFeature(&'a ReliefFeature),
    AbstractReliefComponentKind(AbstractReliefComponentKindRef<'a>),
}

impl<'a> From<&'a AbstractSpaceBoundaryKind> for AbstractSpaceBoundaryKindRef<'a> {
    fn from(item: &'a AbstractSpaceBoundaryKind) -> Self {
        match item {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => {
                Self::AbstractThematicSurfaceKind(x.into())
            }
            AbstractSpaceBoundaryKind::ReliefFeature(x) => Self::ReliefFeature(x),
            AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => {
                Self::AbstractReliefComponentKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractSpaceBoundary for AbstractSpaceBoundaryKindRef<'a> {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        match self {
            Self::AbstractThematicSurfaceKind(x) => x.abstract_space_boundary(),
            Self::ReliefFeature(x) => x.abstract_space_boundary(),
            Self::AbstractReliefComponentKind(x) => x.abstract_space_boundary(),
        }
    }
}
crate::impl_abstract_space_boundary_traits!(AbstractSpaceBoundaryKindRef<'_>);

impl<'a> HasFeatureType for AbstractSpaceBoundaryKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractThematicSurfaceKind(x) => x.feature_type(),
            Self::ReliefFeature(_) => FeatureType::ReliefFeature,
            Self::AbstractReliefComponentKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_boundary_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractSpaceBoundaryKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractSpaceBoundaryKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref!(AbstractSpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_boundary_kind_ref!($variant, $variant);
    };
}
impl_from_for_space_boundary_kind_ref!(AbstractThematicSurfaceKind);
impl_from_for_space_boundary_kind_ref!(ReliefFeature);
impl_from_for_space_boundary_kind_ref!(AbstractReliefComponentKind);

#[macro_export]
macro_rules! impl_try_from_for_space_boundary_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceBoundaryKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceBoundaryKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceBoundaryKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref!(AbstractSpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_boundary_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_space_boundary_kind_ref!(ReliefFeature);

#[macro_export]
macro_rules! impl_try_from_space_boundary_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceBoundaryKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceBoundaryKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceBoundaryKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_for_enum!(AbstractSpaceBoundaryKind, $EnumRef);
    };
}
impl_try_from_city_object_kind_ref_for_enum!(
    AbstractSpaceBoundaryKind,
    AbstractSpaceBoundaryKindRef
);
