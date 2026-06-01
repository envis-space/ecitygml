use crate::impl_abstract_construction_surface_traits;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
    CeilingSurface, FloorSurface, GroundSurface, InteriorWallSurface, OuterCeilingSurface,
    OuterFloorSurface, RoofSurface, WallSurface,
};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionSurfaceKind {
    CeilingSurface(CeilingSurface),
    FloorSurface(FloorSurface),
    GroundSurface(GroundSurface),
    InteriorWallSurface(InteriorWallSurface),
    OuterCeilingSurface(OuterCeilingSurface),
    OuterFloorSurface(OuterFloorSurface),
    RoofSurface(RoofSurface),
    WallSurface(WallSurface),
}

impl ConstructionSurfaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::FloorSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::GroundSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::RoofSurface(x) => x.iter_features(),
            ConstructionSurfaceKind::WallSurface(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::FloorSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::GroundSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::RoofSurface(x) => x.for_each_feature_mut(f),
            ConstructionSurfaceKind::WallSurface(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::FloorSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::GroundSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::RoofSurface(x) => x.compute_envelope(),
            ConstructionSurfaceKind::WallSurface(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::FloorSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::GroundSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::RoofSurface(x) => x.recompute_bounding_shape(),
            ConstructionSurfaceKind::WallSurface(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::FloorSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::GroundSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::RoofSurface(x) => x.apply_transform(m),
            ConstructionSurfaceKind::WallSurface(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractConstructionSurface for ConstructionSurfaceKind {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::FloorSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::GroundSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::RoofSurface(x) => x.abstract_construction_surface(),
            ConstructionSurfaceKind::WallSurface(x) => x.abstract_construction_surface(),
        }
    }
}

impl AsAbstractConstructionSurfaceMut for ConstructionSurfaceKind {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        match self {
            ConstructionSurfaceKind::CeilingSurface(x) => x.abstract_construction_surface_mut(),
            ConstructionSurfaceKind::FloorSurface(x) => x.abstract_construction_surface_mut(),
            ConstructionSurfaceKind::GroundSurface(x) => x.abstract_construction_surface_mut(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            ConstructionSurfaceKind::OuterCeilingSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.abstract_construction_surface_mut(),
            ConstructionSurfaceKind::RoofSurface(x) => x.abstract_construction_surface_mut(),
            ConstructionSurfaceKind::WallSurface(x) => x.abstract_construction_surface_mut(),
        }
    }
}

impl_abstract_construction_surface_traits!(ConstructionSurfaceKind);

impl<'a> From<&'a ConstructionSurfaceKind> for FeatureRef<'a> {
    fn from(item: &'a ConstructionSurfaceKind) -> Self {
        match item {
            ConstructionSurfaceKind::CeilingSurface(x) => x.into(),
            ConstructionSurfaceKind::FloorSurface(x) => x.into(),
            ConstructionSurfaceKind::GroundSurface(x) => x.into(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.into(),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.into(),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.into(),
            ConstructionSurfaceKind::RoofSurface(x) => x.into(),
            ConstructionSurfaceKind::WallSurface(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut ConstructionSurfaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut ConstructionSurfaceKind) -> Self {
        match item {
            ConstructionSurfaceKind::CeilingSurface(x) => x.into(),
            ConstructionSurfaceKind::FloorSurface(x) => x.into(),
            ConstructionSurfaceKind::GroundSurface(x) => x.into(),
            ConstructionSurfaceKind::InteriorWallSurface(x) => x.into(),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => x.into(),
            ConstructionSurfaceKind::OuterFloorSurface(x) => x.into(),
            ConstructionSurfaceKind::RoofSurface(x) => x.into(),
            ConstructionSurfaceKind::WallSurface(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_construction_surface_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::ConstructionSurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::ConstructionSurfaceKind::$type(x)
            }
        }
        $crate::impl_from_for_thematic_surface_kind!(ConstructionSurfaceKind, $type);
    };
}
impl_from_construction_surface_kind!(CeilingSurface);
impl_from_construction_surface_kind!(FloorSurface);
impl_from_construction_surface_kind!(GroundSurface);
impl_from_construction_surface_kind!(InteriorWallSurface);
impl_from_construction_surface_kind!(OuterCeilingSurface);
impl_from_construction_surface_kind!(OuterFloorSurface);
impl_from_construction_surface_kind!(RoofSurface);
impl_from_construction_surface_kind!(WallSurface);
