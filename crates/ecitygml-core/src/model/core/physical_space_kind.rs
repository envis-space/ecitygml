use crate::impl_abstract_physical_space_mut_traits;
use crate::impl_abstract_physical_space_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::occupied_space_kind::OccupiedSpaceKind;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::unoccupied_space_kind::UnoccupiedSpaceKind;
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut,
};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalSpaceKind {
    OccupiedSpaceKind(OccupiedSpaceKind),
    UnoccupiedSpaceKind(UnoccupiedSpaceKind),
}

impl PhysicalSpaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.iter_features(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.for_each_feature_mut(f),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.compute_envelope(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.recompute_bounding_shape(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.apply_transform(m),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractPhysicalSpace for PhysicalSpaceKind {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.abstract_physical_space(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.abstract_physical_space(),
        }
    }
}

impl AsAbstractPhysicalSpaceMut for PhysicalSpaceKind {
    fn abstract_physical_space_mut(&mut self) -> &mut AbstractPhysicalSpace {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.abstract_physical_space_mut(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.abstract_physical_space_mut(),
        }
    }
}

impl_abstract_physical_space_traits!(PhysicalSpaceKind);
impl_abstract_physical_space_mut_traits!(PhysicalSpaceKind);

impl HasFeatureType for PhysicalSpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::OccupiedSpaceKind(x) => x.feature_type(),
            Self::UnoccupiedSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_physical_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::PhysicalSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::PhysicalSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind!(PhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_physical_space_kind!($variant, $variant);
    };
}
impl_from_for_physical_space_kind!(OccupiedSpaceKind);
impl_from_for_physical_space_kind!(UnoccupiedSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_physical_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::PhysicalSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::PhysicalSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::PhysicalSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_kind!(PhysicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_physical_space_kind!($variant, $variant);
    };
}
impl_try_from_for_physical_space_kind!(OccupiedSpaceKind);
impl_try_from_for_physical_space_kind!(UnoccupiedSpaceKind);
