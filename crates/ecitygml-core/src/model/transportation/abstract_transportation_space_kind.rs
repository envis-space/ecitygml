use crate::impl_abstract_transportation_space_mut_traits;
use crate::impl_abstract_transportation_space_traits;

use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
    Intersection, Railway, Road, Section, Square, Track, Waterway,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractTransportationSpaceKind {
    Section(Section),
    Intersection(Intersection),
    Road(Road),
    Track(Track),
    Railway(Railway),
    Waterway(Waterway),
    Square(Square),
}

impl AsAbstractTransportationSpace for AbstractTransportationSpaceKind {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.abstract_transportation_space(),
            AbstractTransportationSpaceKind::Intersection(x) => x.abstract_transportation_space(),
            AbstractTransportationSpaceKind::Road(x) => x.abstract_transportation_space(),
            AbstractTransportationSpaceKind::Track(x) => x.abstract_transportation_space(),
            AbstractTransportationSpaceKind::Railway(x) => x.abstract_transportation_space(),
            AbstractTransportationSpaceKind::Waterway(x) => x.abstract_transportation_space(),
            AbstractTransportationSpaceKind::Square(x) => x.abstract_transportation_space(),
        }
    }
}

impl AsAbstractTransportationSpaceMut for AbstractTransportationSpaceKind {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.abstract_transportation_space_mut(),
            AbstractTransportationSpaceKind::Intersection(x) => {
                x.abstract_transportation_space_mut()
            }
            AbstractTransportationSpaceKind::Road(x) => x.abstract_transportation_space_mut(),
            AbstractTransportationSpaceKind::Track(x) => x.abstract_transportation_space_mut(),
            AbstractTransportationSpaceKind::Railway(x) => x.abstract_transportation_space_mut(),
            AbstractTransportationSpaceKind::Waterway(x) => x.abstract_transportation_space_mut(),
            AbstractTransportationSpaceKind::Square(x) => x.abstract_transportation_space_mut(),
        }
    }
}

impl_abstract_transportation_space_traits!(AbstractTransportationSpaceKind);
impl_abstract_transportation_space_mut_traits!(AbstractTransportationSpaceKind);

impl HasFeatureType for AbstractTransportationSpaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Section(x) => x.feature_type(),
            Self::Intersection(x) => x.feature_type(),
            Self::Road(x) => x.feature_type(),
            Self::Track(x) => x.feature_type(),
            Self::Railway(x) => x.feature_type(),
            Self::Waterway(x) => x.feature_type(),
            Self::Square(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_transportation_space_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::transportation::AbstractTransportationSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::transportation::AbstractTransportationSpaceKind::$type(x)
            }
        }
        $crate::impl_from_for_unoccupied_space_kind!(AbstractTransportationSpaceKind, $type);
    };
}
impl_from_transportation_space_kind!(Section);
impl_from_transportation_space_kind!(Intersection);
impl_from_transportation_space_kind!(Road);
impl_from_transportation_space_kind!(Track);
impl_from_transportation_space_kind!(Railway);
impl_from_transportation_space_kind!(Waterway);
impl_from_transportation_space_kind!(Square);

#[macro_export]
macro_rules! impl_try_from_transportation_space_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::transportation::AbstractTransportationSpaceKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::transportation::AbstractTransportationSpaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::transportation::AbstractTransportationSpaceKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_unoccupied_space_kind!(AbstractTransportationSpaceKind, $type);
    };
}
impl_try_from_transportation_space_kind!(Section);
impl_try_from_transportation_space_kind!(Intersection);
impl_try_from_transportation_space_kind!(Road);
impl_try_from_transportation_space_kind!(Track);
impl_try_from_transportation_space_kind!(Railway);
impl_try_from_transportation_space_kind!(Waterway);
impl_try_from_transportation_space_kind!(Square);

impl IterFeatures for AbstractTransportationSpaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.iter_features(),
            AbstractTransportationSpaceKind::Intersection(x) => x.iter_features(),
            AbstractTransportationSpaceKind::Road(x) => x.iter_features(),
            AbstractTransportationSpaceKind::Track(x) => x.iter_features(),
            AbstractTransportationSpaceKind::Railway(x) => x.iter_features(),
            AbstractTransportationSpaceKind::Waterway(x) => x.iter_features(),
            AbstractTransportationSpaceKind::Square(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractTransportationSpaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.for_each_feature_mut(f),
            AbstractTransportationSpaceKind::Intersection(x) => x.for_each_feature_mut(f),
            AbstractTransportationSpaceKind::Road(x) => x.for_each_feature_mut(f),
            AbstractTransportationSpaceKind::Track(x) => x.for_each_feature_mut(f),
            AbstractTransportationSpaceKind::Railway(x) => x.for_each_feature_mut(f),
            AbstractTransportationSpaceKind::Waterway(x) => x.for_each_feature_mut(f),
            AbstractTransportationSpaceKind::Square(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractTransportationSpaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.compute_envelope(),
            AbstractTransportationSpaceKind::Intersection(x) => x.compute_envelope(),
            AbstractTransportationSpaceKind::Road(x) => x.compute_envelope(),
            AbstractTransportationSpaceKind::Track(x) => x.compute_envelope(),
            AbstractTransportationSpaceKind::Railway(x) => x.compute_envelope(),
            AbstractTransportationSpaceKind::Waterway(x) => x.compute_envelope(),
            AbstractTransportationSpaceKind::Square(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractTransportationSpaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.apply_transform(m),
            AbstractTransportationSpaceKind::Intersection(x) => x.apply_transform(m),
            AbstractTransportationSpaceKind::Road(x) => x.apply_transform(m),
            AbstractTransportationSpaceKind::Track(x) => x.apply_transform(m),
            AbstractTransportationSpaceKind::Railway(x) => x.apply_transform(m),
            AbstractTransportationSpaceKind::Waterway(x) => x.apply_transform(m),
            AbstractTransportationSpaceKind::Square(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.apply_isometry(isometry),
            AbstractTransportationSpaceKind::Intersection(x) => x.apply_isometry(isometry),
            AbstractTransportationSpaceKind::Road(x) => x.apply_isometry(isometry),
            AbstractTransportationSpaceKind::Track(x) => x.apply_isometry(isometry),
            AbstractTransportationSpaceKind::Railway(x) => x.apply_isometry(isometry),
            AbstractTransportationSpaceKind::Waterway(x) => x.apply_isometry(isometry),
            AbstractTransportationSpaceKind::Square(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.apply_translation(vector),
            AbstractTransportationSpaceKind::Intersection(x) => x.apply_translation(vector),
            AbstractTransportationSpaceKind::Road(x) => x.apply_translation(vector),
            AbstractTransportationSpaceKind::Track(x) => x.apply_translation(vector),
            AbstractTransportationSpaceKind::Railway(x) => x.apply_translation(vector),
            AbstractTransportationSpaceKind::Waterway(x) => x.apply_translation(vector),
            AbstractTransportationSpaceKind::Square(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.apply_rotation(rotation),
            AbstractTransportationSpaceKind::Intersection(x) => x.apply_rotation(rotation),
            AbstractTransportationSpaceKind::Road(x) => x.apply_rotation(rotation),
            AbstractTransportationSpaceKind::Track(x) => x.apply_rotation(rotation),
            AbstractTransportationSpaceKind::Railway(x) => x.apply_rotation(rotation),
            AbstractTransportationSpaceKind::Waterway(x) => x.apply_rotation(rotation),
            AbstractTransportationSpaceKind::Square(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractTransportationSpaceKind::Section(x) => x.apply_scale(scale),
            AbstractTransportationSpaceKind::Intersection(x) => x.apply_scale(scale),
            AbstractTransportationSpaceKind::Road(x) => x.apply_scale(scale),
            AbstractTransportationSpaceKind::Track(x) => x.apply_scale(scale),
            AbstractTransportationSpaceKind::Railway(x) => x.apply_scale(scale),
            AbstractTransportationSpaceKind::Waterway(x) => x.apply_scale(scale),
            AbstractTransportationSpaceKind::Square(x) => x.apply_scale(scale),
        }
    }
}
