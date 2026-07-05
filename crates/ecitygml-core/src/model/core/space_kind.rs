use crate::impl_abstract_space_mut_traits;
use crate::impl_abstract_space_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::logical_space_kind::LogicalSpaceKind;
use crate::model::core::physical_space_kind::PhysicalSpaceKind;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
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
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            SpaceKind::LogicalSpaceKind(x) => x.iter_features(),
            SpaceKind::PhysicalSpaceKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
impl_abstract_space_mut_traits!(SpaceKind);

impl HasFeatureType for SpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::LogicalSpaceKind(x) => x.feature_type(),
            Self::PhysicalSpaceKind(x) => x.feature_type(),
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

#[macro_export]
macro_rules! impl_try_from_for_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::SpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::SpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::SpaceKind::$variant(k) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_city_object_kind!(SpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_space_kind!($variant, $variant);
    };
}
impl_try_from_for_space_kind!(LogicalSpaceKind);
impl_try_from_for_space_kind!(PhysicalSpaceKind);
