use crate::impl_abstract_installation_traits;
use crate::model::building::BuildingInstallation;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractInstallation, AsAbstractInstallation, AsAbstractInstallationMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum InstallationKind {
    BuildingInstallation(BuildingInstallation),
}

impl InstallationKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            InstallationKind::BuildingInstallation(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            InstallationKind::BuildingInstallation(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            InstallationKind::BuildingInstallation(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            InstallationKind::BuildingInstallation(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            InstallationKind::BuildingInstallation(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractInstallation for InstallationKind {
    fn abstract_installation(&self) -> &AbstractInstallation {
        match self {
            InstallationKind::BuildingInstallation(x) => x.abstract_installation(),
        }
    }
}

impl AsAbstractInstallationMut for InstallationKind {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation {
        match self {
            InstallationKind::BuildingInstallation(x) => x.abstract_installation_mut(),
        }
    }
}

impl_abstract_installation_traits!(InstallationKind);

impl<'a> From<&'a InstallationKind> for FeatureRef<'a> {
    fn from(item: &'a InstallationKind) -> Self {
        match item {
            InstallationKind::BuildingInstallation(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut InstallationKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut InstallationKind) -> Self {
        match item {
            InstallationKind::BuildingInstallation(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_installation_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::InstallationKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::InstallationKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(InstallationKind, $type);
    };
}
impl_from_installation_kind!(BuildingInstallation);
