use crate::impl_try_from_space_boundary_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::ConstructionSurfaceKind;
use crate::model::construction::FillingSurfaceKind;
use crate::model::construction::refs::{ConstructionSurfaceKindRefMut, FillingSurfaceKindRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
    ClosureSurface, ThematicSurfaceKind,
};
use crate::model::generics::GenericThematicSurface;
use crate::model::land_use::LandUse;
use crate::model::transportation::{AuxiliaryTrafficArea, Marking, TrafficArea};
use crate::model::water_body::{WaterBoundarySurfaceKind, WaterBoundarySurfaceKindRefMut};

#[derive(Debug)]
pub enum ThematicSurfaceKindRefMut<'a> {
    AuxiliaryTrafficArea(&'a mut AuxiliaryTrafficArea),
    ClosureSurface(&'a mut ClosureSurface),
    ConstructionSurfaceKind(ConstructionSurfaceKindRefMut<'a>),
    FillingSurfaceKind(FillingSurfaceKindRefMut<'a>),
    GenericThematicSurface(&'a mut GenericThematicSurface),
    LandUse(&'a mut LandUse),
    Marking(&'a mut Marking),
    TrafficArea(&'a mut TrafficArea),
    WaterBoundarySurfaceKind(WaterBoundarySurfaceKindRefMut<'a>),
}

impl<'a> From<&'a mut ThematicSurfaceKind> for ThematicSurfaceKindRefMut<'a> {
    fn from(item: &'a mut ThematicSurfaceKind) -> Self {
        match item {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => Self::AuxiliaryTrafficArea(x),
            ThematicSurfaceKind::ClosureSurface(x) => Self::ClosureSurface(x),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => {
                Self::ConstructionSurfaceKind(x.into())
            }
            ThematicSurfaceKind::FillingSurfaceKind(x) => Self::FillingSurfaceKind(x.into()),
            ThematicSurfaceKind::GenericThematicSurface(x) => Self::GenericThematicSurface(x),
            ThematicSurfaceKind::LandUse(x) => Self::LandUse(x),
            ThematicSurfaceKind::Marking(x) => Self::Marking(x),
            ThematicSurfaceKind::TrafficArea(x) => Self::TrafficArea(x),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => {
                Self::WaterBoundarySurfaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractThematicSurface for ThematicSurfaceKindRefMut<'a> {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface(),
            Self::ClosureSurface(x) => x.abstract_thematic_surface(),
            Self::ConstructionSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::FillingSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::GenericThematicSurface(x) => x.abstract_thematic_surface(),
            Self::LandUse(x) => x.abstract_thematic_surface(),
            Self::Marking(x) => x.abstract_thematic_surface(),
            Self::TrafficArea(x) => x.abstract_thematic_surface(),
            Self::WaterBoundarySurfaceKind(x) => x.abstract_thematic_surface(),
        }
    }
}

impl<'a> AsAbstractThematicSurfaceMut for ThematicSurfaceKindRefMut<'a> {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface_mut(),
            Self::ClosureSurface(x) => x.abstract_thematic_surface_mut(),
            Self::ConstructionSurfaceKind(x) => x.abstract_thematic_surface_mut(),
            Self::FillingSurfaceKind(x) => x.abstract_thematic_surface_mut(),
            Self::GenericThematicSurface(x) => x.abstract_thematic_surface_mut(),
            Self::LandUse(x) => x.abstract_thematic_surface_mut(),
            Self::Marking(x) => x.abstract_thematic_surface_mut(),
            Self::TrafficArea(x) => x.abstract_thematic_surface_mut(),
            Self::WaterBoundarySurfaceKind(x) => x.abstract_thematic_surface_mut(),
        }
    }
}
crate::impl_abstract_thematic_surface_traits!(ThematicSurfaceKindRefMut<'_>);
crate::impl_abstract_thematic_surface_mut_traits!(ThematicSurfaceKindRefMut<'_>);

impl<'a> ThematicSurfaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.recompute_bounding_shape(),
            Self::ClosureSurface(x) => x.recompute_bounding_shape(),
            Self::ConstructionSurfaceKind(x) => x.recompute_bounding_shape(),
            Self::FillingSurfaceKind(x) => x.recompute_bounding_shape(),
            Self::GenericThematicSurface(x) => x.recompute_bounding_shape(),
            Self::LandUse(x) => x.recompute_bounding_shape(),
            Self::Marking(x) => x.recompute_bounding_shape(),
            Self::TrafficArea(x) => x.recompute_bounding_shape(),
            Self::WaterBoundarySurfaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for ThematicSurfaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.feature_type(),
            Self::ClosureSurface(x) => x.feature_type(),
            Self::ConstructionSurfaceKind(x) => x.feature_type(),
            Self::FillingSurfaceKind(x) => x.feature_type(),
            Self::GenericThematicSurface(x) => x.feature_type(),
            Self::LandUse(x) => x.feature_type(),
            Self::Marking(x) => x.feature_type(),
            Self::TrafficArea(x) => x.feature_type(),
            Self::WaterBoundarySurfaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_thematic_surface_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::ThematicSurfaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::ThematicSurfaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind_ref_mut!(ThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_thematic_surface_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_thematic_surface_kind_ref_mut!(AuxiliaryTrafficArea);
impl_from_for_thematic_surface_kind_ref_mut!(ClosureSurface);
impl_from_for_thematic_surface_kind_ref_mut!(ConstructionSurfaceKind);
impl_from_for_thematic_surface_kind_ref_mut!(FillingSurfaceKind);
impl_from_for_thematic_surface_kind_ref_mut!(GenericThematicSurface);
impl_from_for_thematic_surface_kind_ref_mut!(LandUse);
impl_from_for_thematic_surface_kind_ref_mut!(Marking);
impl_from_for_thematic_surface_kind_ref_mut!(TrafficArea);
impl_from_for_thematic_surface_kind_ref_mut!(WaterBoundarySurfaceKind);

#[macro_export]
macro_rules! impl_try_from_for_thematic_surface_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::ThematicSurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::ThematicSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::ThematicSurfaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind_ref_mut!(ThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_thematic_surface_kind_ref_mut!(AuxiliaryTrafficArea);
impl_try_from_for_thematic_surface_kind_ref_mut!(ClosureSurface);
impl_try_from_for_thematic_surface_kind_ref_mut!(GenericThematicSurface);
impl_try_from_for_thematic_surface_kind_ref_mut!(LandUse);
impl_try_from_for_thematic_surface_kind_ref_mut!(Marking);
impl_try_from_for_thematic_surface_kind_ref_mut!(TrafficArea);

#[macro_export]
macro_rules! impl_try_from_thematic_surface_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::ThematicSurfaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::ThematicSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::ThematicSurfaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_boundary_kind_ref_mut_for_enum!(
            ThematicSurfaceKind,
            $EnumRefMut
        );
    };
}
impl_try_from_space_boundary_kind_ref_mut_for_enum!(ThematicSurfaceKind, ThematicSurfaceKindRefMut);
