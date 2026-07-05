use crate::impl_try_from_surface_data_kind_ref_for_enum;
use crate::model::appearance::{
    AbstractTexture, AsAbstractTexture, GeoreferencedTexture, ParameterizedTexture, TextureKind,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug, Clone, Copy)]
pub enum TextureKindRef<'a> {
    GeoreferencedTexture(&'a GeoreferencedTexture),
    ParameterizedTexture(&'a ParameterizedTexture),
}

impl<'a> From<&'a TextureKind> for TextureKindRef<'a> {
    fn from(item: &'a TextureKind) -> Self {
        match item {
            TextureKind::GeoreferencedTexture(x) => Self::GeoreferencedTexture(x),
            TextureKind::ParameterizedTexture(x) => Self::ParameterizedTexture(x),
        }
    }
}

impl<'a> AsAbstractTexture for TextureKindRef<'a> {
    fn abstract_texture(&self) -> &AbstractTexture {
        match self {
            Self::GeoreferencedTexture(x) => x.abstract_texture(),
            Self::ParameterizedTexture(x) => x.abstract_texture(),
        }
    }
}
crate::impl_abstract_texture_traits!(TextureKindRef<'_>);

impl<'a> HasFeatureType for TextureKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::GeoreferencedTexture(x) => x.feature_type(),
            Self::ParameterizedTexture(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_texture_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::appearance::refs::TextureKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::appearance::refs::TextureKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_surface_data_kind_ref!(TextureKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_texture_kind_ref!($variant, $variant);
    };
}
impl_from_for_texture_kind_ref!(GeoreferencedTexture);
impl_from_for_texture_kind_ref!(ParameterizedTexture);

#[macro_export]
macro_rules! impl_try_from_for_texture_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::TextureKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::TextureKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::TextureKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_texture_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_texture_kind_ref!(GeoreferencedTexture);
impl_try_from_for_texture_kind_ref!(ParameterizedTexture);
impl_try_from_surface_data_kind_ref_for_enum!(TextureKind, TextureKindRef);
