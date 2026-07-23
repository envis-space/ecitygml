use crate::impl_abstract_appearance_mut_traits;
use crate::impl_abstract_appearance_traits;
use crate::model::appearance::Appearance;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractAppearance, AsAbstractAppearance, AsAbstractAppearanceMut};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractAppearanceKind {
    Appearance(Appearance),
}

impl AsAbstractAppearance for AbstractAppearanceKind {
    fn abstract_appearance(&self) -> &AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance(),
        }
    }
}

impl AsAbstractAppearanceMut for AbstractAppearanceKind {
    fn abstract_appearance_mut(&mut self) -> &mut AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance_mut(),
        }
    }
}

impl_abstract_appearance_traits!(AbstractAppearanceKind);
impl_abstract_appearance_mut_traits!(AbstractAppearanceKind);

impl HasFeatureType for AbstractAppearanceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Appearance(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_appearance_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractAppearanceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractAppearanceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind!(AbstractAppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_appearance_kind!($variant, $variant);
    };
}
impl_from_for_appearance_kind!(Appearance);

#[macro_export]
macro_rules! impl_try_from_for_appearance_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractAppearanceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractAppearanceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractAppearanceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind!(AbstractAppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_appearance_kind!($variant, $variant);
    };
}
impl_try_from_for_appearance_kind!(Appearance);

impl IterFeatures for AbstractAppearanceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractAppearanceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractAppearanceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractAppearanceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractAppearanceKind::Appearance(x) => x.apply_scale(scale),
        }
    }
}
