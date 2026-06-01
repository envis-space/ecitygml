use crate::impl_abstract_physical_space_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::occupied_space_kind::OccupiedSpaceKind;
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
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.iter_features(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
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

impl<'a> From<&'a PhysicalSpaceKind> for FeatureRef<'a> {
    fn from(item: &'a PhysicalSpaceKind) -> Self {
        match item {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.into(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a PhysicalSpaceKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a PhysicalSpaceKind) -> Result<Self, ()> {
        match item {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.try_into(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.try_into(),
        }
    }
}

impl<'a> From<&'a mut PhysicalSpaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut PhysicalSpaceKind) -> Self {
        match item {
            PhysicalSpaceKind::OccupiedSpaceKind(x) => x.into(),
            PhysicalSpaceKind::UnoccupiedSpaceKind(x) => x.into(),
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
