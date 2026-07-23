use crate::impl_abstract_building_subdivision_mut_traits;
use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
    BuildingUnit, Storey,
};
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractBuildingSubdivisionKind {
    BuildingUnit(BuildingUnit),
    Storey(Storey),
}

impl AsAbstractBuildingSubdivision for AbstractBuildingSubdivisionKind {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(subdivision) => {
                subdivision.abstract_building_subdivision()
            }
            AbstractBuildingSubdivisionKind::Storey(subdivision) => {
                subdivision.abstract_building_subdivision()
            }
        }
    }
}

impl AsAbstractBuildingSubdivisionMut for AbstractBuildingSubdivisionKind {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(subdivision) => {
                subdivision.abstract_building_subdivision_mut()
            }
            AbstractBuildingSubdivisionKind::Storey(subdivision) => {
                subdivision.abstract_building_subdivision_mut()
            }
        }
    }
}

impl_abstract_building_subdivision_traits!(AbstractBuildingSubdivisionKind);
impl_abstract_building_subdivision_mut_traits!(AbstractBuildingSubdivisionKind);

impl HasFeatureType for AbstractBuildingSubdivisionKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingUnit(x) => x.feature_type(),
            Self::Storey(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_subdivision_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::building::AbstractBuildingSubdivisionKind {
            fn from(x: $type) -> Self {
                $crate::model::building::AbstractBuildingSubdivisionKind::$type(x)
            }
        }
        $crate::impl_from_for_logical_space_kind!(AbstractBuildingSubdivisionKind, $type);
    };
}
impl_from_building_subdivision_kind!(BuildingUnit);
impl_from_building_subdivision_kind!(Storey);

#[macro_export]
macro_rules! impl_try_from_building_subdivision_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::building::AbstractBuildingSubdivisionKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::building::AbstractBuildingSubdivisionKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::building::AbstractBuildingSubdivisionKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_logical_space_kind!(AbstractBuildingSubdivisionKind, $type);
    };
}
impl_try_from_building_subdivision_kind!(BuildingUnit);
impl_try_from_building_subdivision_kind!(Storey);

impl IterFeatures for AbstractBuildingSubdivisionKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.iter_features(),
            AbstractBuildingSubdivisionKind::Storey(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractBuildingSubdivisionKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.for_each_feature_mut(f),
            AbstractBuildingSubdivisionKind::Storey(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractBuildingSubdivisionKind {
    fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.compute_envelope(),
            AbstractBuildingSubdivisionKind::Storey(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractBuildingSubdivisionKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.apply_transform(m),
            AbstractBuildingSubdivisionKind::Storey(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.apply_isometry(isometry),
            AbstractBuildingSubdivisionKind::Storey(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.apply_translation(vector),
            AbstractBuildingSubdivisionKind::Storey(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.apply_rotation(rotation),
            AbstractBuildingSubdivisionKind::Storey(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => x.apply_scale(scale),
            AbstractBuildingSubdivisionKind::Storey(x) => x.apply_scale(scale),
        }
    }
}
