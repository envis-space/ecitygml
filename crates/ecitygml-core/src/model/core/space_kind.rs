use crate::impl_abstract_space_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::logical_space_kind::LogicalSpaceKind;
use crate::model::core::physical_space_kind::PhysicalSpaceKind;
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum SpaceKind {
    LogicalSpaceKind(LogicalSpaceKind),
    PhysicalSpaceKind(PhysicalSpaceKind),
}

impl SpaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.iter_features(),
            SpaceKind::PhysicalSpaceKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.for_each_feature_mut(f),
            SpaceKind::PhysicalSpaceKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.compute_envelope(),
            SpaceKind::PhysicalSpaceKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.recompute_bounding_shape(),
            SpaceKind::PhysicalSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.apply_transform(m),
            SpaceKind::PhysicalSpaceKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractSpace for SpaceKind {
    fn abstract_space(&self) -> &AbstractSpace {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.abstract_space(),
            SpaceKind::PhysicalSpaceKind(x) => x.abstract_space(),
        }
    }
}

impl AsAbstractSpaceMut for SpaceKind {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.abstract_space_mut(),
            SpaceKind::PhysicalSpaceKind(x) => x.abstract_space_mut(),
        }
    }
}

impl_abstract_space_traits!(SpaceKind);

impl<'a> From<&'a SpaceKind> for FeatureRef<'a> {
    fn from(item: &'a SpaceKind) -> Self {
        match item {
            SpaceKind::LogicalSpaceKind(x) => x.into(),
            SpaceKind::PhysicalSpaceKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a SpaceKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a SpaceKind) -> Result<Self, ()> {
        match item {
            SpaceKind::LogicalSpaceKind(x) => x.try_into(),
            SpaceKind::PhysicalSpaceKind(x) => x.try_into(),
        }
    }
}

impl<'a> From<&'a mut SpaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut SpaceKind) -> Self {
        match item {
            SpaceKind::LogicalSpaceKind(x) => x.into(),
            SpaceKind::PhysicalSpaceKind(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::SpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::SpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_city_object_kind!(SpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_space_kind!($variant, $variant);
    };
}
impl_from_for_space_kind!(LogicalSpaceKind);
impl_from_for_space_kind!(PhysicalSpaceKind);
