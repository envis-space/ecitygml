use crate::impl_try_from_thematic_surface_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractFillingSurface, AbstractFillingSurfaceKind, AsAbstractFillingSurface, DoorSurface,
    WindowSurface,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractFillingSurfaceKindRef<'a> {
    DoorSurface(&'a DoorSurface),
    WindowSurface(&'a WindowSurface),
}

impl<'a> From<&'a AbstractFillingSurfaceKind> for AbstractFillingSurfaceKindRef<'a> {
    fn from(item: &'a AbstractFillingSurfaceKind) -> Self {
        match item {
            AbstractFillingSurfaceKind::DoorSurface(x) => Self::DoorSurface(x),
            AbstractFillingSurfaceKind::WindowSurface(x) => Self::WindowSurface(x),
        }
    }
}

impl<'a> AsAbstractFillingSurface for AbstractFillingSurfaceKindRef<'a> {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        match self {
            Self::DoorSurface(x) => x.abstract_filling_surface(),
            Self::WindowSurface(x) => x.abstract_filling_surface(),
        }
    }
}
crate::impl_abstract_filling_surface_traits!(AbstractFillingSurfaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractFillingSurfaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::DoorSurface(_) => FeatureType::DoorSurface,
            Self::WindowSurface(_) => FeatureType::WindowSurface,
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_surface_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type>
            for $crate::model::construction::refs::AbstractFillingSurfaceKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::construction::refs::AbstractFillingSurfaceKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref!(AbstractFillingSurfaceKind, $type);
    };
}
impl_from_filling_surface_kind_ref!(DoorSurface);
impl_from_filling_surface_kind_ref!(WindowSurface);

#[macro_export]
macro_rules! impl_try_from_filling_surface_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::AbstractFillingSurfaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::AbstractFillingSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::AbstractFillingSurfaceKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref!(AbstractFillingSurfaceKind, $type);
    };
}
impl_try_from_filling_surface_kind_ref!(DoorSurface);
impl_try_from_filling_surface_kind_ref!(WindowSurface);
impl_try_from_thematic_surface_kind_ref_for_enum!(
    AbstractFillingSurfaceKind,
    AbstractFillingSurfaceKindRef
);
