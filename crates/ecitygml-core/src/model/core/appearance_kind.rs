use crate::impl_abstract_appearance_mut_traits;
use crate::impl_abstract_appearance_traits;
use crate::model::appearance::Appearance;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{AbstractAppearance, AsAbstractAppearance, AsAbstractAppearanceMut};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum AppearanceKind {
    Appearance(Appearance),
}

impl AppearanceKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            AppearanceKind::Appearance(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AppearanceKind::Appearance(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AppearanceKind::Appearance(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            AppearanceKind::Appearance(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            AppearanceKind::Appearance(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractAppearance for AppearanceKind {
    fn abstract_appearance(&self) -> &AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance(),
        }
    }
}

impl AsAbstractAppearanceMut for AppearanceKind {
    fn abstract_appearance_mut(&mut self) -> &mut AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance_mut(),
        }
    }
}

impl_abstract_appearance_traits!(AppearanceKind);
impl_abstract_appearance_mut_traits!(AppearanceKind);

impl HasFeatureType for AppearanceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Appearance(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_appearance_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AppearanceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AppearanceKind::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_appearance_kind!($variant, $variant);
    };
}
impl_from_for_appearance_kind!(Appearance);

#[macro_export]
macro_rules! impl_try_from_for_appearance_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AppearanceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AppearanceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AppearanceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind!(AppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_appearance_kind!($variant, $variant);
    };
}
impl_try_from_for_appearance_kind!(Appearance);
