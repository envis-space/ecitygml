use crate::impl_abstract_texture_mut_traits;
use crate::impl_abstract_texture_traits;
use crate::model::appearance::{
    AbstractTexture, AsAbstractTexture, AsAbstractTextureMut, GeoreferencedTexture,
    ParameterizedTexture,
};
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractTextureKind {
    GeoreferencedTexture(GeoreferencedTexture),
    ParameterizedTexture(ParameterizedTexture),
}

impl AsAbstractTexture for AbstractTextureKind {
    fn abstract_texture(&self) -> &AbstractTexture {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.abstract_texture(),
            AbstractTextureKind::ParameterizedTexture(x) => x.abstract_texture(),
        }
    }
}

impl AsAbstractTextureMut for AbstractTextureKind {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.abstract_texture_mut(),
            AbstractTextureKind::ParameterizedTexture(x) => x.abstract_texture_mut(),
        }
    }
}

impl_abstract_texture_traits!(AbstractTextureKind);
impl_abstract_texture_mut_traits!(AbstractTextureKind);

impl HasFeatureType for AbstractTextureKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::GeoreferencedTexture(x) => x.feature_type(),
            Self::ParameterizedTexture(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_texture_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::appearance::AbstractTextureKind {
            fn from(x: $type) -> Self {
                $crate::model::appearance::AbstractTextureKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_surface_data_kind!(AbstractTextureKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_texture_kind!($variant, $variant);
    };
}
impl_from_for_texture_kind!(GeoreferencedTexture);
impl_from_for_texture_kind!(ParameterizedTexture);

#[macro_export]
macro_rules! impl_try_from_for_texture_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::appearance::AbstractTextureKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::appearance::AbstractTextureKind) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::AbstractTextureKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_surface_data_kind!(AbstractTextureKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_texture_kind!($variant, $variant);
    };
}
impl_try_from_for_texture_kind!(GeoreferencedTexture);
impl_try_from_for_texture_kind!(ParameterizedTexture);

impl IterFeatures for AbstractTextureKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.iter_features(),
            AbstractTextureKind::ParameterizedTexture(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractTextureKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.for_each_feature_mut(f),
            AbstractTextureKind::ParameterizedTexture(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractTextureKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.compute_envelope(),
            AbstractTextureKind::ParameterizedTexture(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractTextureKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.apply_transform(m),
            AbstractTextureKind::ParameterizedTexture(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.apply_isometry(isometry),
            AbstractTextureKind::ParameterizedTexture(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.apply_translation(vector),
            AbstractTextureKind::ParameterizedTexture(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.apply_rotation(rotation),
            AbstractTextureKind::ParameterizedTexture(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractTextureKind::GeoreferencedTexture(x) => x.apply_scale(scale),
            AbstractTextureKind::ParameterizedTexture(x) => x.apply_scale(scale),
        }
    }
}
