use crate::impl_abstract_thematic_surface_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::construction::{ConstructionSurfaceKind, FillingSurfaceKind};
use crate::model::core::closure_surface::ClosureSurface;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use crate::model::transportation::{AuxiliaryTrafficArea, Marking, TrafficArea};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ThematicSurfaceKind {
    AuxiliaryTrafficArea(AuxiliaryTrafficArea),
    ClosureSurface(ClosureSurface),
    ConstructionSurfaceKind(ConstructionSurfaceKind),
    FillingSurfaceKind(FillingSurfaceKind),
    Marking(Marking),
    TrafficArea(TrafficArea),
}

impl ThematicSurfaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.iter_features(),
            ThematicSurfaceKind::ClosureSurface(x) => x.iter_features(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.iter_features(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.iter_features(),
            ThematicSurfaceKind::Marking(x) => x.iter_features(),
            ThematicSurfaceKind::TrafficArea(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::ClosureSurface(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::Marking(x) => x.for_each_feature_mut(f),
            ThematicSurfaceKind::TrafficArea(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.compute_envelope(),
            ThematicSurfaceKind::ClosureSurface(x) => x.compute_envelope(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.compute_envelope(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.compute_envelope(),
            ThematicSurfaceKind::Marking(x) => x.compute_envelope(),
            ThematicSurfaceKind::TrafficArea(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::ClosureSurface(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::Marking(x) => x.recompute_bounding_shape(),
            ThematicSurfaceKind::TrafficArea(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.apply_transform(m),
            ThematicSurfaceKind::ClosureSurface(x) => x.apply_transform(m),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.apply_transform(m),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.apply_transform(m),
            ThematicSurfaceKind::Marking(x) => x.apply_transform(m),
            ThematicSurfaceKind::TrafficArea(x) => x.apply_transform(m),
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
            ThematicSurfaceKind::Marking(x) => x.abstract_thematic_surface(),
            ThematicSurfaceKind::TrafficArea(x) => x.abstract_thematic_surface(),
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
            ThematicSurfaceKind::Marking(x) => x.abstract_thematic_surface_mut(),
            ThematicSurfaceKind::TrafficArea(x) => x.abstract_thematic_surface_mut(),
        }
    }
}

impl_abstract_thematic_surface_traits!(ThematicSurfaceKind);

impl<'a> From<&'a ThematicSurfaceKind> for FeatureRef<'a> {
    fn from(item: &'a ThematicSurfaceKind) -> Self {
        match item {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.into(),
            ThematicSurfaceKind::ClosureSurface(x) => x.into(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.into(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.into(),
            ThematicSurfaceKind::Marking(x) => x.into(),
            ThematicSurfaceKind::TrafficArea(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a ThematicSurfaceKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a ThematicSurfaceKind) -> Result<Self, ()> {
        match item {
            ThematicSurfaceKind::AuxiliaryTrafficArea(_) => Err(()),
            ThematicSurfaceKind::ClosureSurface(_) => Err(()),
            ThematicSurfaceKind::ConstructionSurfaceKind(_) => Err(()),
            ThematicSurfaceKind::FillingSurfaceKind(_) => Err(()),
            ThematicSurfaceKind::Marking(_) => Err(()),
            ThematicSurfaceKind::TrafficArea(_) => Err(()),
            // TODO: GenericThematicSurfaceKind
        }
    }
}

impl<'a> From<&'a mut ThematicSurfaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut ThematicSurfaceKind) -> Self {
        match item {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => x.into(),
            ThematicSurfaceKind::ClosureSurface(x) => x.into(),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => x.into(),
            ThematicSurfaceKind::FillingSurfaceKind(x) => x.into(),
            ThematicSurfaceKind::Marking(x) => x.into(),
            ThematicSurfaceKind::TrafficArea(x) => x.into(),
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
impl_from_for_thematic_surface_kind!(Marking);
impl_from_for_thematic_surface_kind!(TrafficArea);
impl_from_for_thematic_surface_kind!(ConstructionSurfaceKind);
impl_from_for_thematic_surface_kind!(FillingSurfaceKind);
