use crate::impl_try_from_thematic_surface_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut, DoorSurface,
    FillingSurfaceKind, WindowSurface,
};

#[derive(Debug)]
pub enum FillingSurfaceKindRefMut<'a> {
    DoorSurface(&'a mut DoorSurface),
    WindowSurface(&'a mut WindowSurface),
}

impl<'a> From<&'a mut FillingSurfaceKind> for FillingSurfaceKindRefMut<'a> {
    fn from(item: &'a mut FillingSurfaceKind) -> Self {
        match item {
            FillingSurfaceKind::DoorSurface(x) => Self::DoorSurface(x),
            FillingSurfaceKind::WindowSurface(x) => Self::WindowSurface(x),
        }
    }
}

impl<'a> AsAbstractFillingSurface for FillingSurfaceKindRefMut<'a> {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        match self {
            Self::DoorSurface(x) => x.abstract_filling_surface(),
            Self::WindowSurface(x) => x.abstract_filling_surface(),
        }
    }
}

impl<'a> AsAbstractFillingSurfaceMut for FillingSurfaceKindRefMut<'a> {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        match self {
            Self::DoorSurface(x) => x.abstract_filling_surface_mut(),
            Self::WindowSurface(x) => x.abstract_filling_surface_mut(),
        }
    }
}
crate::impl_abstract_filling_surface_traits!(FillingSurfaceKindRefMut<'_>);
crate::impl_abstract_filling_surface_mut_traits!(FillingSurfaceKindRefMut<'_>);

impl<'a> FillingSurfaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::DoorSurface(x) => x.recompute_bounding_shape(),
            Self::WindowSurface(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for FillingSurfaceKindRefMut<'a> {
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
            for $crate::model::construction::refs::FillingSurfaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::FillingSurfaceKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref_mut!(FillingSurfaceKind, $type);
    };
}
impl_from_filling_surface_kind_ref_mut!(DoorSurface);
impl_from_filling_surface_kind_ref_mut!(WindowSurface);

#[macro_export]
macro_rules! impl_try_from_filling_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::FillingSurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::FillingSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::FillingSurfaceKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!(FillingSurfaceKind, $type);
    };
}
impl_try_from_filling_surface_kind_ref_mut!(DoorSurface);
impl_try_from_filling_surface_kind_ref_mut!(WindowSurface);
impl_try_from_thematic_surface_kind_ref_mut_for_enum!(FillingSurfaceKind, FillingSurfaceKindRefMut);
