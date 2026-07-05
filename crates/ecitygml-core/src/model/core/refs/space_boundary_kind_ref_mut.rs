use crate::impl_try_from_city_object_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::ThematicSurfaceKind;
use crate::model::core::refs::ThematicSurfaceKindRefMut;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut, SpaceBoundaryKind,
};
use crate::model::relief::ReliefComponentKind;
use crate::model::relief::ReliefFeature;
use crate::model::relief::refs::ReliefComponentKindRefMut;

#[derive(Debug)]
pub enum SpaceBoundaryKindRefMut<'a> {
    ThematicSurfaceKind(ThematicSurfaceKindRefMut<'a>),
    ReliefFeature(&'a mut ReliefFeature),
    ReliefComponentKind(ReliefComponentKindRefMut<'a>),
}

impl<'a> From<&'a mut SpaceBoundaryKind> for SpaceBoundaryKindRefMut<'a> {
    fn from(item: &'a mut SpaceBoundaryKind) -> Self {
        match item {
            SpaceBoundaryKind::ThematicSurfaceKind(x) => Self::ThematicSurfaceKind(x.into()),
            SpaceBoundaryKind::ReliefFeature(x) => Self::ReliefFeature(x),
            SpaceBoundaryKind::ReliefComponentKind(x) => Self::ReliefComponentKind(x.into()),
        }
    }
}

impl<'a> AsAbstractSpaceBoundary for SpaceBoundaryKindRefMut<'a> {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        match self {
            Self::ThematicSurfaceKind(x) => x.abstract_space_boundary(),
            Self::ReliefFeature(x) => x.abstract_space_boundary(),
            Self::ReliefComponentKind(x) => x.abstract_space_boundary(),
        }
    }
}

impl<'a> AsAbstractSpaceBoundaryMut for SpaceBoundaryKindRefMut<'a> {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary {
        match self {
            Self::ThematicSurfaceKind(x) => x.abstract_space_boundary_mut(),
            Self::ReliefFeature(x) => x.abstract_space_boundary_mut(),
            Self::ReliefComponentKind(x) => x.abstract_space_boundary_mut(),
        }
    }
}
crate::impl_abstract_space_boundary_traits!(SpaceBoundaryKindRefMut<'_>);
crate::impl_abstract_space_boundary_mut_traits!(SpaceBoundaryKindRefMut<'_>);

impl<'a> SpaceBoundaryKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::ThematicSurfaceKind(x) => x.recompute_bounding_shape(),
            Self::ReliefFeature(x) => x.recompute_bounding_shape(),
            Self::ReliefComponentKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for SpaceBoundaryKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::ThematicSurfaceKind(x) => x.feature_type(),
            Self::ReliefFeature(x) => x.feature_type(),
            Self::ReliefComponentKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_boundary_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::SpaceBoundaryKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::SpaceBoundaryKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind_ref_mut!(SpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_boundary_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_space_boundary_kind_ref_mut!(ThematicSurfaceKind);
impl_from_for_space_boundary_kind_ref_mut!(ReliefFeature);
impl_from_for_space_boundary_kind_ref_mut!(ReliefComponentKind);

#[macro_export]
macro_rules! impl_try_from_for_space_boundary_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceBoundaryKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::SpaceBoundaryKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceBoundaryKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind_ref_mut!(SpaceBoundaryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_boundary_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_space_boundary_kind_ref_mut!(ReliefFeature);

#[macro_export]
macro_rules! impl_try_from_space_boundary_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::SpaceBoundaryKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::SpaceBoundaryKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::SpaceBoundaryKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_city_object_kind_ref_mut_for_enum!(SpaceBoundaryKind, $EnumRefMut);
    };
}
impl_try_from_city_object_kind_ref_mut_for_enum!(SpaceBoundaryKind, SpaceBoundaryKindRefMut);
