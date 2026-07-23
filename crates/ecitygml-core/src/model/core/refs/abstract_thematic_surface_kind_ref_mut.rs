use crate::impl_try_from_space_boundary_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::AbstractConstructionSurfaceKind;
use crate::model::construction::AbstractFillingSurfaceKind;
use crate::model::construction::refs::{
    AbstractConstructionSurfaceKindRefMut, AbstractFillingSurfaceKindRefMut,
};
use crate::model::core::{
    AbstractThematicSurface, AbstractThematicSurfaceKind, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut, ClosureSurface,
};
use crate::model::generics::GenericThematicSurface;
use crate::model::land_use::LandUse;
use crate::model::transportation::{AuxiliaryTrafficArea, HoleSurface, Marking, TrafficArea};
use crate::model::water_body::{
    AbstractWaterBoundarySurfaceKind, AbstractWaterBoundarySurfaceKindRefMut,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractThematicSurfaceKindRefMut<'a> {
    AuxiliaryTrafficArea(&'a mut AuxiliaryTrafficArea),
    ClosureSurface(&'a mut ClosureSurface),
    AbstractConstructionSurfaceKind(AbstractConstructionSurfaceKindRefMut<'a>),
    AbstractFillingSurfaceKind(AbstractFillingSurfaceKindRefMut<'a>),
    GenericThematicSurface(&'a mut GenericThematicSurface),
    HoleSurface(&'a mut HoleSurface),
    LandUse(&'a mut LandUse),
    Marking(&'a mut Marking),
    TrafficArea(&'a mut TrafficArea),
    AbstractWaterBoundarySurfaceKind(AbstractWaterBoundarySurfaceKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractThematicSurfaceKind> for AbstractThematicSurfaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractThematicSurfaceKind) -> Self {
        match item {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => Self::AuxiliaryTrafficArea(x),
            AbstractThematicSurfaceKind::ClosureSurface(x) => Self::ClosureSurface(x),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                Self::AbstractConstructionSurfaceKind(x.into())
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                Self::AbstractFillingSurfaceKind(x.into())
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => {
                Self::GenericThematicSurface(x)
            }
            AbstractThematicSurfaceKind::HoleSurface(x) => Self::HoleSurface(x),
            AbstractThematicSurfaceKind::LandUse(x) => Self::LandUse(x),
            AbstractThematicSurfaceKind::Marking(x) => Self::Marking(x),
            AbstractThematicSurfaceKind::TrafficArea(x) => Self::TrafficArea(x),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                Self::AbstractWaterBoundarySurfaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractThematicSurface for AbstractThematicSurfaceKindRefMut<'a> {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface(),
            Self::ClosureSurface(x) => x.abstract_thematic_surface(),
            Self::AbstractConstructionSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::AbstractFillingSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::GenericThematicSurface(x) => x.abstract_thematic_surface(),
            Self::HoleSurface(x) => x.abstract_thematic_surface(),
            Self::LandUse(x) => x.abstract_thematic_surface(),
            Self::Marking(x) => x.abstract_thematic_surface(),
            Self::TrafficArea(x) => x.abstract_thematic_surface(),
            Self::AbstractWaterBoundarySurfaceKind(x) => x.abstract_thematic_surface(),
        }
    }
}

impl<'a> AsAbstractThematicSurfaceMut for AbstractThematicSurfaceKindRefMut<'a> {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface_mut(),
            Self::ClosureSurface(x) => x.abstract_thematic_surface_mut(),
            Self::AbstractConstructionSurfaceKind(x) => x.abstract_thematic_surface_mut(),
            Self::AbstractFillingSurfaceKind(x) => x.abstract_thematic_surface_mut(),
            Self::GenericThematicSurface(x) => x.abstract_thematic_surface_mut(),
            Self::HoleSurface(x) => x.abstract_thematic_surface_mut(),
            Self::LandUse(x) => x.abstract_thematic_surface_mut(),
            Self::Marking(x) => x.abstract_thematic_surface_mut(),
            Self::TrafficArea(x) => x.abstract_thematic_surface_mut(),
            Self::AbstractWaterBoundarySurfaceKind(x) => x.abstract_thematic_surface_mut(),
        }
    }
}
crate::impl_abstract_thematic_surface_traits!(AbstractThematicSurfaceKindRefMut<'_>);
crate::impl_abstract_thematic_surface_mut_traits!(AbstractThematicSurfaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractThematicSurfaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.feature_type(),
            Self::ClosureSurface(x) => x.feature_type(),
            Self::AbstractConstructionSurfaceKind(x) => x.feature_type(),
            Self::AbstractFillingSurfaceKind(x) => x.feature_type(),
            Self::GenericThematicSurface(x) => x.feature_type(),
            Self::HoleSurface(x) => x.feature_type(),
            Self::LandUse(x) => x.feature_type(),
            Self::Marking(x) => x.feature_type(),
            Self::TrafficArea(x) => x.feature_type(),
            Self::AbstractWaterBoundarySurfaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_thematic_surface_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractThematicSurfaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractThematicSurfaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind_ref_mut!(AbstractThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_thematic_surface_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_thematic_surface_kind_ref_mut!(AuxiliaryTrafficArea);
impl_from_for_thematic_surface_kind_ref_mut!(ClosureSurface);
impl_from_for_thematic_surface_kind_ref_mut!(AbstractConstructionSurfaceKind);
impl_from_for_thematic_surface_kind_ref_mut!(AbstractFillingSurfaceKind);
impl_from_for_thematic_surface_kind_ref_mut!(GenericThematicSurface);
impl_from_for_thematic_surface_kind_ref_mut!(HoleSurface);
impl_from_for_thematic_surface_kind_ref_mut!(LandUse);
impl_from_for_thematic_surface_kind_ref_mut!(Marking);
impl_from_for_thematic_surface_kind_ref_mut!(TrafficArea);
impl_from_for_thematic_surface_kind_ref_mut!(AbstractWaterBoundarySurfaceKind);

#[macro_export]
macro_rules! impl_try_from_for_thematic_surface_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractThematicSurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractThematicSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractThematicSurfaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind_ref_mut!(AbstractThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_thematic_surface_kind_ref_mut!(AuxiliaryTrafficArea);
impl_try_from_for_thematic_surface_kind_ref_mut!(ClosureSurface);
impl_try_from_for_thematic_surface_kind_ref_mut!(GenericThematicSurface);
impl_try_from_for_thematic_surface_kind_ref_mut!(HoleSurface);
impl_try_from_for_thematic_surface_kind_ref_mut!(LandUse);
impl_try_from_for_thematic_surface_kind_ref_mut!(Marking);
impl_try_from_for_thematic_surface_kind_ref_mut!(TrafficArea);

#[macro_export]
macro_rules! impl_try_from_thematic_surface_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractThematicSurfaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractThematicSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractThematicSurfaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_boundary_kind_ref_mut_for_enum!(
            AbstractThematicSurfaceKind,
            $EnumRefMut
        );
    };
}
impl_try_from_space_boundary_kind_ref_mut_for_enum!(
    AbstractThematicSurfaceKind,
    AbstractThematicSurfaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractThematicSurfaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.recompute_bounding_shape(),
            Self::ClosureSurface(x) => x.recompute_bounding_shape(),
            Self::AbstractConstructionSurfaceKind(x) => x.recompute_bounding_shape(),
            Self::AbstractFillingSurfaceKind(x) => x.recompute_bounding_shape(),
            Self::GenericThematicSurface(x) => x.recompute_bounding_shape(),
            Self::HoleSurface(x) => x.recompute_bounding_shape(),
            Self::LandUse(x) => x.recompute_bounding_shape(),
            Self::Marking(x) => x.recompute_bounding_shape(),
            Self::TrafficArea(x) => x.recompute_bounding_shape(),
            Self::AbstractWaterBoundarySurfaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}
