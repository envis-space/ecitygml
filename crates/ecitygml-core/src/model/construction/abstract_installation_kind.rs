use crate::impl_abstract_installation_mut_traits;
use crate::impl_abstract_installation_traits;
use crate::model::building::BuildingInstallation;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{
    AbstractInstallation, AsAbstractInstallation, AsAbstractInstallationMut,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractInstallationKind {
    BuildingInstallation(BuildingInstallation),
}

impl AsAbstractInstallation for AbstractInstallationKind {
    fn abstract_installation(&self) -> &AbstractInstallation {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.abstract_installation(),
        }
    }
}

impl AsAbstractInstallationMut for AbstractInstallationKind {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.abstract_installation_mut(),
        }
    }
}

impl_abstract_installation_traits!(AbstractInstallationKind);
impl_abstract_installation_mut_traits!(AbstractInstallationKind);

impl HasFeatureType for AbstractInstallationKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingInstallation(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_installation_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::AbstractInstallationKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::AbstractInstallationKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(AbstractInstallationKind, $type);
    };
}
impl_from_installation_kind!(BuildingInstallation);

#[macro_export]
macro_rules! impl_try_from_installation_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::construction::AbstractInstallationKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::AbstractInstallationKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::AbstractInstallationKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(AbstractInstallationKind, $type);
    };
}
impl_try_from_installation_kind!(BuildingInstallation);

impl IterFeatures for AbstractInstallationKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractInstallationKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractInstallationKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractInstallationKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractInstallationKind::BuildingInstallation(x) => x.apply_scale(scale),
        }
    }
}
