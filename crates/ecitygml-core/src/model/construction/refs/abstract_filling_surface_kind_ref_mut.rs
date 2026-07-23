use crate::impl_try_from_thematic_surface_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractFillingSurface, AbstractFillingSurfaceKind, AsAbstractFillingSurface,
    AsAbstractFillingSurfaceMut, DoorSurface, WindowSurface,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractFillingSurfaceKindRefMut<'a> {
    DoorSurface(&'a mut DoorSurface),
    WindowSurface(&'a mut WindowSurface),
}

impl<'a> From<&'a mut AbstractFillingSurfaceKind> for AbstractFillingSurfaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractFillingSurfaceKind) -> Self {
        match item {
            AbstractFillingSurfaceKind::DoorSurface(x) => Self::DoorSurface(x),
            AbstractFillingSurfaceKind::WindowSurface(x) => Self::WindowSurface(x),
        }
    }
}

impl<'a> AsAbstractFillingSurface for AbstractFillingSurfaceKindRefMut<'a> {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        match self {
            Self::DoorSurface(x) => x.abstract_filling_surface(),
            Self::WindowSurface(x) => x.abstract_filling_surface(),
        }
    }
}

impl<'a> AsAbstractFillingSurfaceMut for AbstractFillingSurfaceKindRefMut<'a> {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        match self {
            Self::DoorSurface(x) => x.abstract_filling_surface_mut(),
            Self::WindowSurface(x) => x.abstract_filling_surface_mut(),
        }
    }
}
crate::impl_abstract_filling_surface_traits!(AbstractFillingSurfaceKindRefMut<'_>);
crate::impl_abstract_filling_surface_mut_traits!(AbstractFillingSurfaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractFillingSurfaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::DoorSurface(x) => x.feature_type(),
            Self::WindowSurface(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::construction::refs::AbstractFillingSurfaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::AbstractFillingSurfaceKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref_mut!(AbstractFillingSurfaceKind, $type);
    };
}
impl_from_filling_surface_kind_ref_mut!(DoorSurface);
impl_from_filling_surface_kind_ref_mut!(WindowSurface);

#[macro_export]
macro_rules! impl_try_from_filling_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::AbstractFillingSurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::AbstractFillingSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::AbstractFillingSurfaceKindRefMut::$type(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!(AbstractFillingSurfaceKind, $type);
    };
}
impl_try_from_filling_surface_kind_ref_mut!(DoorSurface);
impl_try_from_filling_surface_kind_ref_mut!(WindowSurface);
impl_try_from_thematic_surface_kind_ref_mut_for_enum!(
    AbstractFillingSurfaceKind,
    AbstractFillingSurfaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractFillingSurfaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::DoorSurface(x) => x.recompute_bounding_shape(),
            Self::WindowSurface(x) => x.recompute_bounding_shape(),
        }
    }
}
