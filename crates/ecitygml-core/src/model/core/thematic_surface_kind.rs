use crate::impl_abstract_thematic_surface_mut_traits;
use crate::impl_abstract_thematic_surface_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::construction::{ConstructionSurfaceKind, FillingSurfaceKind};
use crate::model::core::closure_surface::ClosureSurface;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use crate::model::generics::GenericThematicSurface;
use crate::model::land_use::LandUse;
use crate::model::transportation::{AuxiliaryTrafficArea, Marking, TrafficArea};
use crate::model::water_body::WaterBoundarySurfaceKind;
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ThematicSurfaceKind {
    AuxiliaryTrafficArea(AuxiliaryTrafficArea),
    ClosureSurface(ClosureSurface),
    ConstructionSurfaceKind(ConstructionSurfaceKind),
    FillingSurfaceKind(FillingSurfaceKind),
    GenericThematicSurface(GenericThematicSurface),
    LandUse(LandUse),
    Marking(Marking),
    TrafficArea(TrafficArea),
    WaterBoundarySurfaceKind(WaterBoundarySurfaceKind),
}

impl ThematicSurfaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.iter_features(),
            ThematicSurfaceKind::ClosureSurface(x) => x.iter_features(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.iter_features(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.iter_features(),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.iter_features(),
            ThematicSurfaceKind::LandUse(x) => x.iter_features(),
            ThematicSurfaceKind::Marking(x) => x.iter_features(),
            ThematicSurfaceKind::TrafficArea(x) => x.iter_features(),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::ClosureSurface(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::LandUse(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::Marking(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::TrafficArea(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.compute_envelope(),
            ThematicSurfaceKind::ClosureSurface(x) => x.compute_envelope(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.compute_envelope(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.compute_envelope(),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.compute_envelope(),
            ThematicSurfaceKind::LandUse(x) => x.compute_envelope(),
            ThematicSurfaceKind::Marking(x) => x.compute_envelope(),
            ThematicSurfaceKind::TrafficArea(x) => x.compute_envelope(),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::ClosureSurface(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::LandUse(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::Marking(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::TrafficArea(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_transform(m),
            ThematicSurfaceKind::ClosureSurface(x) => x.apply_transform(m),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.apply_transform(m),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.apply_transform(m),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.apply_transform(m),
            ThematicSurfaceKind::LandUse(x) => x.apply_transform(m),
            ThematicSurfaceKind::Marking(x) => x.apply_transform(m),
            ThematicSurfaceKind::TrafficArea(x) => x.apply_transform(m),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractThematicSurface for ThematicSurfaceKind {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::ClosureSurface(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::LandUse(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::Marking(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::TrafficArea(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.abstract_thematic_surface(),
        }
    }
}

impl AsAbstractThematicSurfaceMut for ThematicSurfaceKind {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::ClosureSurface(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::GenericThematicSurface(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::LandUse(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::Marking(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::TrafficArea(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => x.abstract_thematic_surface_mut(),
        }
    }
}

impl_abstract_thematic_surface_traits!(ThematicSurfaceKind);
impl_abstract_thematic_surface_mut_traits!(ThematicSurfaceKind);

impl HasFeatureType for ThematicSurfaceKind {
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
macro_rules! impl_from_for_thematic_surface_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::ThematicSurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::ThematicSurfaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind!(ThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_thematic_surface_kind!($variant, $variant);
    };
}
impl_from_for_thematic_surface_kind!(AuxiliaryTrafficArea);
impl_from_for_thematic_surface_kind!(ClosureSurface);
impl_from_for_thematic_surface_kind!(ConstructionSurfaceKind);
impl_from_for_thematic_surface_kind!(FillingSurfaceKind);
impl_from_for_thematic_surface_kind!(GenericThematicSurface);
impl_from_for_thematic_surface_kind!(LandUse);
impl_from_for_thematic_surface_kind!(Marking);
impl_from_for_thematic_surface_kind!(TrafficArea);
impl_from_for_thematic_surface_kind!(WaterBoundarySurfaceKind);

#[macro_export]
macro_rules! impl_try_from_for_thematic_surface_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::ThematicSurfaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::ThematicSurfaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::ThematicSurfaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind!(ThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_thematic_surface_kind!($variant, $variant);
    };
}
impl_try_from_for_thematic_surface_kind!(AuxiliaryTrafficArea);
impl_try_from_for_thematic_surface_kind!(ClosureSurface);
impl_try_from_for_thematic_surface_kind!(GenericThematicSurface);
impl_try_from_for_thematic_surface_kind!(LandUse);
impl_try_from_for_thematic_surface_kind!(Marking);
impl_try_from_for_thematic_surface_kind!(TrafficArea);
