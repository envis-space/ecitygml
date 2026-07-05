use crate::impl_try_from_surface_data_kind_ref_mut_for_enum;
use crate::model::appearance::{
    AbstractTexture, AsAbstractTexture, AsAbstractTextureMut, GeoreferencedTexture,
    ParameterizedTexture, TextureKind,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug)]
pub enum TextureKindRefMut<'a> {
    GeoreferencedTexture(&'a mut GeoreferencedTexture),
    ParameterizedTexture(&'a mut ParameterizedTexture),
}

impl<'a> From<&'a mut TextureKind> for TextureKindRefMut<'a> {
    fn from(item: &'a mut TextureKind) -> Self {
        match item {
            TextureKind::GeoreferencedTexture(x) => Self::GeoreferencedTexture(x),
            TextureKind::ParameterizedTexture(x) => Self::ParameterizedTexture(x),
        }
    }
}

impl<'a> AsAbstractTexture for TextureKindRefMut<'a> {
    fn abstract_texture(&self) -> &AbstractTexture {
        match self {
            Self::GeoreferencedTexture(x) => x.abstract_texture(),
            Self::ParameterizedTexture(x) => x.abstract_texture(),
        }
    }
}

impl<'a> AsAbstractTextureMut for TextureKindRefMut<'a> {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        match self {
            Self::GeoreferencedTexture(x) => x.abstract_texture_mut(),
            Self::ParameterizedTexture(x) => x.abstract_texture_mut(),
        }
    }
}
crate::impl_abstract_texture_traits!(TextureKindRefMut<'_>);
crate::impl_abstract_texture_mut_traits!(TextureKindRefMut<'_>);

impl<'a> TextureKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::GeoreferencedTexture(x) => x.recompute_bounding_shape(),
            Self::ParameterizedTexture(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for TextureKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::GeoreferencedTexture(x) => x.feature_type(),
            Self::ParameterizedTexture(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_texture_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::appearance::refs::TextureKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::appearance::refs::TextureKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_surface_data_kind_ref_mut!(TextureKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_texture_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_texture_kind_ref_mut!(GeoreferencedTexture);
impl_from_for_texture_kind_ref_mut!(ParameterizedTexture);

#[macro_export]
macro_rules! impl_try_from_for_texture_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::TextureKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::TextureKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::TextureKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_texture_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_texture_kind_ref_mut!(GeoreferencedTexture);
impl_try_from_for_texture_kind_ref_mut!(ParameterizedTexture);
impl_try_from_surface_data_kind_ref_mut_for_enum!(TextureKind, TextureKindRefMut);
