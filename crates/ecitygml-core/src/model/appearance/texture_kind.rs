use crate::impl_abstract_texture_mut_traits;
use crate::impl_abstract_texture_traits;
use crate::model::appearance::{
    AbstractTexture, AsAbstractTexture, AsAbstractTextureMut, GeoreferencedTexture,
    ParameterizedTexture,
};
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum TextureKind {
    GeoreferencedTexture(GeoreferencedTexture),
    ParameterizedTexture(ParameterizedTexture),
}

impl TextureKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.iter_features(),
            TextureKind::ParameterizedTexture(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.for_each_feature_mut(f),
            TextureKind::ParameterizedTexture(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.compute_envelope(),
            TextureKind::ParameterizedTexture(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.recompute_bounding_shape(),
            TextureKind::ParameterizedTexture(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.apply_transform(m),
            TextureKind::ParameterizedTexture(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractTexture for TextureKind {
    fn abstract_texture(&self) -> &AbstractTexture {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.abstract_texture(),
            TextureKind::ParameterizedTexture(x) => x.abstract_texture(),
        }
    }
}

impl AsAbstractTextureMut for TextureKind {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        match self {
            TextureKind::GeoreferencedTexture(x) => x.abstract_texture_mut(),
            TextureKind::ParameterizedTexture(x) => x.abstract_texture_mut(),
        }
    }
}

impl_abstract_texture_traits!(TextureKind);
impl_abstract_texture_mut_traits!(TextureKind);

impl HasFeatureType for TextureKind {
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
        impl From<$type> for $crate::model::appearance::TextureKind {
            fn from(x: $type) -> Self {
                $crate::model::appearance::TextureKind::$variant(x.into())
            }
        }
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
        impl TryFrom<$crate::model::appearance::TextureKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::appearance::TextureKind) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::TextureKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_surface_data_kind!(TextureKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_texture_kind!($variant, $variant);
    };
}
impl_try_from_for_texture_kind!(GeoreferencedTexture);
impl_try_from_for_texture_kind!(ParameterizedTexture);
