use crate::impl_abstract_filling_surface_mut_traits;
use crate::impl_abstract_filling_surface_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut, DoorSurface,
    WindowSurface,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractFillingSurfaceKind {
    DoorSurface(DoorSurface),
    WindowSurface(WindowSurface),
}

impl AsAbstractFillingSurface for AbstractFillingSurfaceKind {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(surface) => surface.abstract_filling_surface(),
            AbstractFillingSurfaceKind::WindowSurface(surface) => {
                surface.abstract_filling_surface()
            }
        }
    }
}

impl AsAbstractFillingSurfaceMut for AbstractFillingSurfaceKind {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(surface) => {
                surface.abstract_filling_surface_mut()
            }
            AbstractFillingSurfaceKind::WindowSurface(surface) => {
                surface.abstract_filling_surface_mut()
            }
        }
    }
}

impl_abstract_filling_surface_traits!(AbstractFillingSurfaceKind);
impl_abstract_filling_surface_mut_traits!(AbstractFillingSurfaceKind);

impl HasFeatureType for AbstractFillingSurfaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::DoorSurface(x) => x.feature_type(),
            Self::WindowSurface(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_surface_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::AbstractFillingSurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::AbstractFillingSurfaceKind::$type(x)
            }
        }
        $crate::impl_from_for_thematic_surface_kind!(AbstractFillingSurfaceKind, $type);
    };
}
impl_from_filling_surface_kind!(DoorSurface);
impl_from_filling_surface_kind!(WindowSurface);

#[macro_export]
macro_rules! impl_try_from_filling_surface_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::construction::AbstractFillingSurfaceKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::AbstractFillingSurfaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::AbstractFillingSurfaceKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind!(AbstractFillingSurfaceKind, $type);
    };
}
impl_try_from_filling_surface_kind!(DoorSurface);
impl_try_from_filling_surface_kind!(WindowSurface);

impl IterFeatures for AbstractFillingSurfaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.iter_features(),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractFillingSurfaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.for_each_feature_mut(f),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractFillingSurfaceKind {
    fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.compute_envelope(),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractFillingSurfaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.apply_transform(m),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.apply_isometry(isometry),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.apply_translation(vector),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.apply_rotation(rotation),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractFillingSurfaceKind::DoorSurface(x) => x.apply_scale(scale),
            AbstractFillingSurfaceKind::WindowSurface(x) => x.apply_scale(scale),
        }
    }
}
