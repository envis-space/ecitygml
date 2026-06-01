use crate::impl_abstract_filling_surface_traits;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut, DoorSurface,
    WindowSurface,
};
use auto_enums::auto_enum;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum FillingSurfaceKind {
    DoorSurface(DoorSurface),
    WindowSurface(WindowSurface),
}

impl FillingSurfaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            FillingSurfaceKind::DoorSurface(x) => x.iter_features(),
            FillingSurfaceKind::WindowSurface(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            FillingSurfaceKind::DoorSurface(x) => x.for_each_feature_mut(f),
            FillingSurfaceKind::WindowSurface(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            FillingSurfaceKind::DoorSurface(x) => x.compute_envelope(),
            FillingSurfaceKind::WindowSurface(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            FillingSurfaceKind::DoorSurface(x) => x.recompute_bounding_shape(),
            FillingSurfaceKind::WindowSurface(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            FillingSurfaceKind::DoorSurface(x) => x.apply_transform(m),
            FillingSurfaceKind::WindowSurface(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractFillingSurface for FillingSurfaceKind {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        match self {
            FillingSurfaceKind::DoorSurface(surface) => surface.abstract_filling_surface(),
            FillingSurfaceKind::WindowSurface(surface) => surface.abstract_filling_surface(),
        }
    }
}

impl AsAbstractFillingSurfaceMut for FillingSurfaceKind {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        match self {
            FillingSurfaceKind::DoorSurface(surface) => surface.abstract_filling_surface_mut(),
            FillingSurfaceKind::WindowSurface(surface) => surface.abstract_filling_surface_mut(),
        }
    }
}

impl_abstract_filling_surface_traits!(FillingSurfaceKind);

impl<'a> From<&'a FillingSurfaceKind> for FeatureRef<'a> {
    fn from(item: &'a FillingSurfaceKind) -> Self {
        match item {
            FillingSurfaceKind::DoorSurface(x) => x.into(),
            FillingSurfaceKind::WindowSurface(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut FillingSurfaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut FillingSurfaceKind) -> Self {
        match item {
            FillingSurfaceKind::DoorSurface(x) => x.into(),
            FillingSurfaceKind::WindowSurface(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_surface_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::FillingSurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::FillingSurfaceKind::$type(x)
            }
        }
        $crate::impl_from_for_thematic_surface_kind!(FillingSurfaceKind, $type);
    };
}
impl_from_filling_surface_kind!(DoorSurface);
impl_from_filling_surface_kind!(WindowSurface);
