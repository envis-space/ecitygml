use crate::impl_abstract_thematic_surface_mut_traits;
use crate::impl_abstract_thematic_surface_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{AbstractConstructionSurfaceKind, AbstractFillingSurfaceKind};
use crate::model::core::closure_surface::ClosureSurface;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use crate::model::generics::GenericThematicSurface;
use crate::model::land_use::LandUse;
use crate::model::transportation::{AuxiliaryTrafficArea, HoleSurface, Marking, TrafficArea};
use crate::model::water_body::AbstractWaterBoundarySurfaceKind;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractThematicSurfaceKind {
    AuxiliaryTrafficArea(AuxiliaryTrafficArea),
    ClosureSurface(ClosureSurface),
    AbstractConstructionSurfaceKind(AbstractConstructionSurfaceKind),
    AbstractFillingSurfaceKind(AbstractFillingSurfaceKind),
    GenericThematicSurface(GenericThematicSurface),
    HoleSurface(HoleSurface),
    LandUse(LandUse),
    Marking(Marking),
    TrafficArea(TrafficArea),
    AbstractWaterBoundarySurfaceKind(AbstractWaterBoundarySurfaceKind),
}

impl AsAbstractThematicSurface for AbstractThematicSurfaceKind {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                x.abstract_thematic_surface()
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                x.abstract_thematic_surface()
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::LandUse(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::Marking(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.abstract_thematic_surface(),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.abstract_thematic_surface()
            }
        }
    }
}

impl AsAbstractThematicSurfaceMut for AbstractThematicSurfaceKind {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => {
                x.abstract_thematic_surface_mut()
            }
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.abstract_thematic_surface_mut(),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                x.abstract_thematic_surface_mut()
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                x.abstract_thematic_surface_mut()
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => {
                x.abstract_thematic_surface_mut()
            }
            AbstractThematicSurfaceKind::HoleSurface(x) => x.abstract_thematic_surface_mut(),
            AbstractThematicSurfaceKind::LandUse(x) => x.abstract_thematic_surface_mut(),
            AbstractThematicSurfaceKind::Marking(x) => x.abstract_thematic_surface_mut(),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.abstract_thematic_surface_mut(),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.abstract_thematic_surface_mut()
            }
        }
    }
}

impl_abstract_thematic_surface_traits!(AbstractThematicSurfaceKind);
impl_abstract_thematic_surface_mut_traits!(AbstractThematicSurfaceKind);

impl HasFeatureType for AbstractThematicSurfaceKind {
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
macro_rules! impl_from_for_thematic_surface_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractThematicSurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractThematicSurfaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind!(AbstractThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_thematic_surface_kind!($variant, $variant);
    };
}
impl_from_for_thematic_surface_kind!(AuxiliaryTrafficArea);
impl_from_for_thematic_surface_kind!(ClosureSurface);
impl_from_for_thematic_surface_kind!(AbstractConstructionSurfaceKind);
impl_from_for_thematic_surface_kind!(AbstractFillingSurfaceKind);
impl_from_for_thematic_surface_kind!(GenericThematicSurface);
impl_from_for_thematic_surface_kind!(HoleSurface);
impl_from_for_thematic_surface_kind!(LandUse);
impl_from_for_thematic_surface_kind!(Marking);
impl_from_for_thematic_surface_kind!(TrafficArea);
impl_from_for_thematic_surface_kind!(AbstractWaterBoundarySurfaceKind);

#[macro_export]
macro_rules! impl_try_from_for_thematic_surface_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractThematicSurfaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractThematicSurfaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractThematicSurfaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind!(AbstractThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_thematic_surface_kind!($variant, $variant);
    };
}
impl_try_from_for_thematic_surface_kind!(AuxiliaryTrafficArea);
impl_try_from_for_thematic_surface_kind!(ClosureSurface);
impl_try_from_for_thematic_surface_kind!(GenericThematicSurface);
impl_try_from_for_thematic_surface_kind!(HoleSurface);
impl_try_from_for_thematic_surface_kind!(LandUse);
impl_try_from_for_thematic_surface_kind!(Marking);
impl_try_from_for_thematic_surface_kind!(TrafficArea);

impl IterFeatures for AbstractThematicSurfaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.iter_features(),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.iter_features(),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => x.iter_features(),
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => x.iter_features(),
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.iter_features(),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.iter_features(),
            AbstractThematicSurfaceKind::LandUse(x) => x.iter_features(),
            AbstractThematicSurfaceKind::Marking(x) => x.iter_features(),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.iter_features(),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractThematicSurfaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                x.for_each_feature_mut(f)
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::LandUse(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::Marking(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.for_each_feature_mut(f),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.for_each_feature_mut(f)
            }
        }
    }
}

impl ComputeEnvelope for AbstractThematicSurfaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::LandUse(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::Marking(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.compute_envelope(),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.compute_envelope()
            }
        }
    }
}

impl ApplyTransform for AbstractThematicSurfaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::LandUse(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::Marking(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.apply_transform(m),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.apply_transform(m)
            }
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                x.apply_isometry(isometry)
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                x.apply_isometry(isometry)
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::LandUse(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::Marking(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.apply_isometry(isometry),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.apply_isometry(isometry)
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                x.apply_translation(vector)
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                x.apply_translation(vector)
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::LandUse(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::Marking(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.apply_translation(vector),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                x.apply_rotation(rotation)
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                x.apply_rotation(rotation)
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::LandUse(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::Marking(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.apply_rotation(rotation),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.apply_rotation(rotation)
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::ClosureSurface(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::HoleSurface(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::LandUse(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::Marking(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::TrafficArea(x) => x.apply_scale(scale),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                x.apply_scale(scale)
            }
        }
    }
}
