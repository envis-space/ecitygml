use crate::impl_try_from_occupied_space_kind_ref_mut_for_enum;
use crate::model::building::BuildingInstallation;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractInstallation, AbstractInstallationKind, AsAbstractInstallation,
    AsAbstractInstallationMut,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractInstallationKindRefMut<'a> {
    BuildingInstallation(&'a mut BuildingInstallation),
}

impl<'a> From<&'a mut AbstractInstallationKind> for AbstractInstallationKindRefMut<'a> {
    fn from(item: &'a mut AbstractInstallationKind) -> Self {
        match item {
            AbstractInstallationKind::BuildingInstallation(x) => Self::BuildingInstallation(x),
        }
    }
}

impl<'a> AsAbstractInstallation for AbstractInstallationKindRefMut<'a> {
    fn abstract_installation(&self) -> &AbstractInstallation {
        match self {
            Self::BuildingInstallation(x) => x.abstract_installation(),
        }
    }
}

impl<'a> AsAbstractInstallationMut for AbstractInstallationKindRefMut<'a> {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation {
        match self {
            Self::BuildingInstallation(x) => x.abstract_installation_mut(),
        }
    }
}
crate::impl_abstract_installation_traits!(AbstractInstallationKindRefMut<'_>);
crate::impl_abstract_installation_mut_traits!(AbstractInstallationKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractInstallationKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingInstallation(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_installation_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::construction::refs::AbstractInstallationKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::AbstractInstallationKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref_mut!(AbstractInstallationKind, $type);
    };
}
impl_from_installation_kind_ref_mut!(BuildingInstallation);

#[macro_export]
macro_rules! impl_try_from_installation_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::AbstractInstallationKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::AbstractInstallationKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::AbstractInstallationKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!(AbstractInstallationKind, $type);
    };
}
impl_try_from_installation_kind_ref_mut!(BuildingInstallation);
impl_try_from_occupied_space_kind_ref_mut_for_enum!(
    AbstractInstallationKind,
    AbstractInstallationKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractInstallationKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::BuildingInstallation(x) => x.recompute_bounding_shape(),
        }
    }
}
