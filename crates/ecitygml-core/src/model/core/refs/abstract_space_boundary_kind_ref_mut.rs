use crate::impl_try_from_city_object_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::AbstractThematicSurfaceKind;
use crate::model::core::refs::AbstractThematicSurfaceKindRefMut;
use crate::model::core::{
    AbstractSpaceBoundary, AbstractSpaceBoundaryKind, AsAbstractSpaceBoundary,
    AsAbstractSpaceBoundaryMut,
};
use crate::model::relief::AbstractReliefComponentKind;
use crate::model::relief::ReliefFeature;
use crate::model::relief::refs::AbstractReliefComponentKindRefMut;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractSpaceBoundaryKindRefMut<'a> {
    AbstractThematicSurfaceKind(AbstractThematicSurfaceKindRefMut<'a>),
    ReliefFeature(&'a mut ReliefFeature),
    AbstractReliefComponentKind(AbstractReliefComponentKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractSpaceBoundaryKind> for AbstractSpaceBoundaryKindRefMut<'a> {
    fn from(item: &'a mut AbstractSpaceBoundaryKind) -> Self {
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

impl<'a> AsAbstractSpaceBoundary for AbstractSpaceBoundaryKindRefMut<'a> {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        match self {
            Self::AbstractThematicSurfaceKind(x) => x.abstract_space_boundary(),
            Self::ReliefFeature(x) => x.abstract_space_boundary(),
            Self::AbstractReliefComponentKind(x) => x.abstract_space_boundary(),
        }
    }
}

impl<'a> AsAbstractSpaceBoundaryMut for AbstractSpaceBoundaryKindRefMut<'a> {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary {
        match self {
            Self::AbstractThematicSurfaceKind(x) => x.abstract_space_boundary_mut(),
            Self::ReliefFeature(x) => x.abstract_space_boundary_mut(),
            Self::AbstractReliefComponentKind(x) => x.abstract_space_boundary_mut(),
        }
    }
}
crate::impl_abstract_space_boundary_traits!(AbstractSpaceBoundaryKindRefMut<'_>);
crate::impl_abstract_space_boundary_mut_traits!(AbstractSpaceBoundaryKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractSpaceBoundaryKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractThematicSurfaceKind(x) => x.feature_type(),
            Self::ReliefFeature(x) => x.feature_type(),
            Self::AbstractReliefComponentKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_boundary_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractSpaceBoundaryKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractSpaceBoundaryKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref_mut!(AbstractSpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_boundary_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_space_boundary_kind_ref_mut!(AbstractThematicSurfaceKind);
impl_from_for_space_boundary_kind_ref_mut!(ReliefFeature);
impl_from_for_space_boundary_kind_ref_mut!(AbstractReliefComponentKind);

#[macro_export]
macro_rules! impl_try_from_for_space_boundary_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceBoundaryKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceBoundaryKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceBoundaryKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref_mut!(AbstractSpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_boundary_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_space_boundary_kind_ref_mut!(ReliefFeature);

#[macro_export]
macro_rules! impl_try_from_space_boundary_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractSpaceBoundaryKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractSpaceBoundaryKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractSpaceBoundaryKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_mut_for_enum!(
            AbstractSpaceBoundaryKind,
            $EnumRefMut
        );
    };
}
impl_try_from_city_object_kind_ref_mut_for_enum!(
    AbstractSpaceBoundaryKind,
    AbstractSpaceBoundaryKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractSpaceBoundaryKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractThematicSurfaceKind(x) => x.recompute_bounding_shape(),
            Self::ReliefFeature(x) => x.recompute_bounding_shape(),
            Self::AbstractReliefComponentKind(x) => x.recompute_bounding_shape(),
        }
    }
}
