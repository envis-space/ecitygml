use crate::impl_abstract_logical_space_traits;
use crate::model::building::BuildingSubdivisionKind;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalSpaceKind {
    BuildingSubdivisionKind(BuildingSubdivisionKind),
}

impl LogicalSpaceKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractLogicalSpace for LogicalSpaceKind {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.abstract_logical_space(),
        }
    }
}

impl AsAbstractLogicalSpaceMut for LogicalSpaceKind {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace {
        match self {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.abstract_logical_space_mut(),
        }
    }
}

impl_abstract_logical_space_traits!(LogicalSpaceKind);

impl<'a> From<&'a LogicalSpaceKind> for FeatureRef<'a> {
    fn from(item: &'a LogicalSpaceKind) -> Self {
        match item {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a LogicalSpaceKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a LogicalSpaceKind) -> Result<Self, ()> {
        match item {
            LogicalSpaceKind::BuildingSubdivisionKind(_x) => Err(()),
            // TODO: GenericLogicalSpace
        }
    }
}

impl<'a> From<&'a mut LogicalSpaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut LogicalSpaceKind) -> Self {
        match item {
            LogicalSpaceKind::BuildingSubdivisionKind(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_logical_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::LogicalSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::LogicalSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_kind!(LogicalSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_logical_space_kind!($variant, $variant);
    };
}
impl_from_for_logical_space_kind!(BuildingSubdivisionKind);
