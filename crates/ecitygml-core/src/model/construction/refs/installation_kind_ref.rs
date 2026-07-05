use crate::impl_try_from_occupied_space_kind_ref_for_enum;
use crate::model::building::BuildingInstallation;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{AbstractInstallation, AsAbstractInstallation, InstallationKind};

#[derive(Debug, Clone, Copy)]
pub enum InstallationKindRef<'a> {
    BuildingInstallation(&'a BuildingInstallation),
}

impl<'a> From<&'a InstallationKind> for InstallationKindRef<'a> {
    fn from(item: &'a InstallationKind) -> Self {
        match item {
            InstallationKind::BuildingInstallation(x) => Self::BuildingInstallation(x),
        }
    }
}

impl<'a> AsAbstractInstallation for InstallationKindRef<'a> {
    fn abstract_installation(&self) -> &AbstractInstallation {
        match self {
            Self::BuildingInstallation(x) => x.abstract_installation(),
        }
    }
}
crate::impl_abstract_installation_traits!(InstallationKindRef<'_>);

impl<'a> HasFeatureType for InstallationKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::BuildingInstallation(_) => FeatureType::BuildingInstallation,
        }
    }
}

#[macro_export]
macro_rules! impl_from_installation_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::construction::refs::InstallationKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::construction::refs::InstallationKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref!(InstallationKind, $type);
    };
}
impl_from_installation_kind_ref!(BuildingInstallation);

#[macro_export]
macro_rules! impl_try_from_installation_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::InstallationKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::InstallationKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::InstallationKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref!(InstallationKind, $type);
    };
}
impl_try_from_installation_kind_ref!(BuildingInstallation);
impl_try_from_occupied_space_kind_ref_for_enum!(InstallationKind, InstallationKindRef);
