use crate::impl_abstract_transportation_space_mut_traits;
use crate::impl_abstract_transportation_space_traits;

use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
    Intersection, Road, Section,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum TransportationSpaceKind {
    Section(Section),
    Intersection(Intersection),
    Road(Road),
}

impl TransportationSpaceKind {
    #[auto_enums::auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            TransportationSpaceKind::Section(x) => x.iter_features(),
            TransportationSpaceKind::Intersection(x) => x.iter_features(),
            TransportationSpaceKind::Road(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            TransportationSpaceKind::Section(x) => x.for_each_feature_mut(f),
            TransportationSpaceKind::Intersection(x) => x.for_each_feature_mut(f),
            TransportationSpaceKind::Road(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            TransportationSpaceKind::Section(x) => x.compute_envelope(),
            TransportationSpaceKind::Intersection(x) => x.compute_envelope(),
            TransportationSpaceKind::Road(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            TransportationSpaceKind::Section(x) => x.recompute_bounding_shape(),
            TransportationSpaceKind::Intersection(x) => x.recompute_bounding_shape(),
            TransportationSpaceKind::Road(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            TransportationSpaceKind::Section(x) => x.apply_transform(m),
            TransportationSpaceKind::Intersection(x) => x.apply_transform(m),
            TransportationSpaceKind::Road(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractTransportationSpace for TransportationSpaceKind {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        match self {
            TransportationSpaceKind::Section(x) => x.abstract_transportation_space(),
            TransportationSpaceKind::Intersection(x) => x.abstract_transportation_space(),
            TransportationSpaceKind::Road(x) => x.abstract_transportation_space(),
        }
    }
}

impl AsAbstractTransportationSpaceMut for TransportationSpaceKind {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        match self {
            TransportationSpaceKind::Section(x) => x.abstract_transportation_space_mut(),
            TransportationSpaceKind::Intersection(x) => x.abstract_transportation_space_mut(),
            TransportationSpaceKind::Road(x) => x.abstract_transportation_space_mut(),
        }
    }
}

impl_abstract_transportation_space_traits!(TransportationSpaceKind);
impl_abstract_transportation_space_mut_traits!(TransportationSpaceKind);

impl HasFeatureType for TransportationSpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Section(x) => x.feature_type(),
            Self::Intersection(x) => x.feature_type(),
            Self::Road(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_transportation_space_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::transportation::TransportationSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::transportation::TransportationSpaceKind::$type(x)
            }
        }
        $crate::impl_from_for_unoccupied_space_kind!(TransportationSpaceKind, $type);
    };
}
impl_from_transportation_space_kind!(Section);
impl_from_transportation_space_kind!(Intersection);
impl_from_transportation_space_kind!(Road);

#[macro_export]
macro_rules! impl_try_from_transportation_space_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::transportation::TransportationSpaceKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::transportation::TransportationSpaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::transportation::TransportationSpaceKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_unoccupied_space_kind!(TransportationSpaceKind, $type);
    };
}
impl_try_from_transportation_space_kind!(Section);
impl_try_from_transportation_space_kind!(Intersection);
impl_try_from_transportation_space_kind!(Road);
