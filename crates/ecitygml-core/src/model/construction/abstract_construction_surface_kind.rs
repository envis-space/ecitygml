use crate::impl_abstract_construction_surface_mut_traits;
use crate::impl_abstract_construction_surface_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
    CeilingSurface, FloorSurface, GroundSurface, InteriorWallSurface, OuterCeilingSurface,
    OuterFloorSurface, RoofSurface, WallSurface,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractConstructionSurfaceKind {
    CeilingSurface(CeilingSurface),
    FloorSurface(FloorSurface),
    GroundSurface(GroundSurface),
    InteriorWallSurface(InteriorWallSurface),
    OuterCeilingSurface(OuterCeilingSurface),
    OuterFloorSurface(OuterFloorSurface),
    RoofSurface(RoofSurface),
    WallSurface(WallSurface),
}

impl AsAbstractConstructionSurface for AbstractConstructionSurfaceKind {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.abstract_construction_surface(),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.abstract_construction_surface(),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.abstract_construction_surface(),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => {
                x.abstract_construction_surface()
            }
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => {
                x.abstract_construction_surface()
            }
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => {
                x.abstract_construction_surface()
            }
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.abstract_construction_surface(),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.abstract_construction_surface(),
        }
    }
}

impl AsAbstractConstructionSurfaceMut for AbstractConstructionSurfaceKind {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::FloorSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::GroundSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::RoofSurface(x) => {
                x.abstract_construction_surface_mut()
            }
            AbstractConstructionSurfaceKind::WallSurface(x) => {
                x.abstract_construction_surface_mut()
            }
        }
    }
}

impl_abstract_construction_surface_traits!(AbstractConstructionSurfaceKind);
impl_abstract_construction_surface_mut_traits!(AbstractConstructionSurfaceKind);

impl HasFeatureType for AbstractConstructionSurfaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::CeilingSurface(x) => x.feature_type(),
            Self::FloorSurface(x) => x.feature_type(),
            Self::GroundSurface(x) => x.feature_type(),
            Self::InteriorWallSurface(x) => x.feature_type(),
            Self::OuterCeilingSurface(x) => x.feature_type(),
            Self::OuterFloorSurface(x) => x.feature_type(),
            Self::RoofSurface(x) => x.feature_type(),
            Self::WallSurface(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_construction_surface_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::AbstractConstructionSurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::AbstractConstructionSurfaceKind::$type(x)
            }
        }
        $crate::impl_from_for_thematic_surface_kind!(AbstractConstructionSurfaceKind, $type);
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

#[macro_export]
macro_rules! impl_try_from_construction_surface_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::construction::AbstractConstructionSurfaceKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::AbstractConstructionSurfaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::AbstractConstructionSurfaceKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind!(AbstractConstructionSurfaceKind, $type);
    };
}
impl_try_from_construction_surface_kind!(CeilingSurface);
impl_try_from_construction_surface_kind!(FloorSurface);
impl_try_from_construction_surface_kind!(GroundSurface);
impl_try_from_construction_surface_kind!(InteriorWallSurface);
impl_try_from_construction_surface_kind!(OuterCeilingSurface);
impl_try_from_construction_surface_kind!(OuterFloorSurface);
impl_try_from_construction_surface_kind!(RoofSurface);
impl_try_from_construction_surface_kind!(WallSurface);

impl IterFeatures for AbstractConstructionSurfaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.iter_features(),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractConstructionSurfaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.for_each_feature_mut(f),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractConstructionSurfaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.compute_envelope(),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractConstructionSurfaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.apply_transform(m),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.apply_isometry(isometry),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.apply_translation(vector),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.apply_rotation(rotation),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::FloorSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::GroundSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::RoofSurface(x) => x.apply_scale(scale),
            AbstractConstructionSurfaceKind::WallSurface(x) => x.apply_scale(scale),
        }
    }
}
